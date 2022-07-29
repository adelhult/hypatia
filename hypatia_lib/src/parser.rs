use crate::Error;
use chumsky::{prelude::*, Stream};
use std::fmt;

/// Parse some source text into an abstract syntax tree of Expr nodes
pub fn parse(source: &str) -> Result<Spanned<Expr>, Vec<Error>> {
    let (tokens, lexing_errors) = lexer().parse_recovery(source);

    // Convert the lexing errors into the Hypatia errors
    let lexing_errors = lexing_errors
        .into_iter()
        .map(|err| err.map(|c| c.to_string()))
        .map(Error::Parsing);

    // return the errors if we can't continue with parsing
    if tokens.is_none() {
        return Err(lexing_errors.collect());
    }

    // Parse the stream of tokens
    let len = source.chars().count();
    let (ast, parsing_errors) =
        parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.unwrap().into_iter()));

    // If there are errors, return them
    if parsing_errors.len() + lexing_errors.len() > 0 {
        return Err(lexing_errors
            .chain(
                parsing_errors
                    .into_iter()
                    .map(|err| Error::Parsing(err.map(|token| token.to_string()))),
            )
            .collect());
    }

    // Or if everything was successful, return the ast!
    // Note: the unwrap is safe since chumsky parse_recovery promises that there
    // will be at least one error if it fails to produce a ast
    Ok(ast.unwrap())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    Ident(String),
    Number(String),
    Bool(bool),
    Unit,
    Update,
    If,
    Else,
    Nothing,
    Add,
    Sub,
    Mul,
    Div,
    Assignment,
    Equal,
    NotEqual,
    Lt,
    Gt,
    Gte,
    Lte,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Semicolon,
    Comma,
    Newline,
    Comment,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "{}", x),
            Token::Number(x) => write!(f, "{}", x),
            Token::Bool(x) => write!(f, "{}", x),
            Token::Unit => write!(f, "unit"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Update => write!(f, "update"),
            Token::Nothing => write!(f, "nothing"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Assignment => write!(f, "="),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Gte => write!(f, ">="),
            Token::Lte => write!(f, "<="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Newline => writeln!(f),
            Token::Comment => write!(f, ""),
        }
    }
}

fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    // parse number
    let frac = just('.').chain(text::digits(10));

    // 13(.37)?
    let decimal_form = text::int(10).chain::<char, _, _>(frac.or_not().flatten());

    let number = decimal_form.or(frac).collect::<String>().map(Token::Number);

    // operators
    let ops = select! {
        '=' => Token::Assignment,
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        '<' => Token::Lt,
        '>' => Token::Gt,
    }
    .or(just("<=").to(Token::Lte))
    .or(just(">=").to(Token::Gte))
    .or(just("==").to(Token::Equal))
    .or(just("!=").to(Token::NotEqual));

    // Control characters
    let control = select! {
        '(' => Token::LParen,
        ')' => Token::RParen,
        '{' => Token::LCurly,
        '}' => Token::RCurly,
        '[' => Token::LBracket,
        ']' => Token::RBracket,
        ';' => Token::Semicolon,
        ',' => Token::Comma,
    }
    .or(text::newline().to(Token::Newline));

    // TODO: support more then just c idents
    let ident = text::ident().map(|i: String| match i.as_str() {
        "unit" => Token::Unit,
        "if" => Token::If,
        "else" => Token::Else,
        "update" => Token::Update,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "nothing" => Token::Nothing,
        s => Token::Ident(s.into()),
    });

    let comment = just("//").then(take_until(just('\n'))).to(Token::Comment);

    let token = comment
        .or(number)
        .or(control)
        .or(ops)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let whitespace = just(' ').or(just('\t')).repeated();

    token
        .map_with_span(|token, span| (token, span))
        .padded_by(whitespace)
        .repeated()
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Error,
    Value(Value),
    Variable(String),
    VarDeclaration(String, Box<Spanned<Self>>),
    VarUpdate(String, Box<Spanned<Self>>),
    Call(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Block(Vec<Spanned<Self>>),
    Program(Vec<Spanned<Self>>),
    BinOp(BinOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nothing,
    Bool(bool),
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, "nothing"),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Number(n) => write!(f, "{n}"),
        }
    }
}

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

/// Parses a stream of tokens and create a AST
///
/// Inspired by: <https://github.com/zesterer/chumsky/blob/master/examples/nano_rust.rs>
fn parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    let separator = just(Token::Newline)
        .or(just(Token::Semicolon))
        .or(just(Token::Comment))
        .repeated()
        .at_least(1);

    let expr = recursive(|expr| {
        let value = select! {
            Token::Nothing => Expr::Value(Value::Nothing),
            Token::Number(n) => Expr::Value(Value::Number(n.parse().unwrap())),
            Token::Bool(x) => Expr::Value(Value::Bool(x)),
        }
        .labelled("value");

        let ident = select! {Token::Ident(i) => i}.labelled("identifier");

        // foo, 20.3, bar,
        let items = expr
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing();

        // General variable assignment syntax
        // x = 20
        let assignment = ident
            .then_ignore(just(Token::Assignment))
            .then(expr.clone());

        // Syntax for updating a variable
        // update x = 20
        let var_update = just(Token::Update)
            .ignore_then(assignment.clone())
            .map(|(name, value)| Expr::VarUpdate(name, Box::new(value)));

        // Syntax for declaring new variables
        // x = 20
        let var_declaration =
            assignment.map(|(name, value)| Expr::VarDeclaration(name, Box::new(value)));

        let atom = value
            .or(var_update)
            .or(var_declaration)
            .or(ident.map(Expr::Variable))
            .map_with_span(|expr, span| (expr, span))
            // Expression surrounded with parentheses
            .or(expr
                .clone()
                .delimited_by(just(Token::LParen), just(Token::RParen)))
            // Attempt to recover anything that looks like a parenthesised expression but contains errors
            .recover_with(nested_delimiters(
                Token::LParen,
                Token::RParen,
                [(Token::LCurly, Token::RCurly)],
                |span| (Expr::Error, span),
            ));

        // A function call f(x)
        let call = atom
            .then(
                items
                    .delimited_by(just(Token::LParen), just(Token::RParen))
                    .map_with_span(|args, span: Span| (args, span))
                    .repeated(),
            )
            .foldl(|f, args| {
                let span = f.1.start..args.1.end;
                (Expr::Call(Box::new(f), args.0), span)
            });

        // Product operators '*' and '/'
        let op = just(Token::Mul)
            .to(BinOp::Mul)
            .or(just(Token::Div).to(BinOp::Div));

        let product = call
            .clone()
            .then(op.then(call).repeated())
            .foldl(|a, (operator, b)| {
                let span = a.1.start..b.1.end;
                (Expr::BinOp(operator, Box::new(a), Box::new(b)), span)
            });

        // Sum operators '+' and '-'
        let op = just(Token::Add)
            .to(BinOp::Add)
            .or(just(Token::Sub).to(BinOp::Sub));
        let sum = product
            .clone()
            .then(op.then(product).repeated())
            .foldl(|a, (operator, b)| {
                let span = a.1.start..b.1.end;
                (Expr::BinOp(operator, Box::new(a), Box::new(b)), span)
            });
        // FIXME: unary operators and comparison

        // multiple expressions separated by line breaks or ";".
        let expressions = expr
            .clone()
            .separated_by(separator.clone())
            .allow_trailing()
            .allow_leading();

        let block = expressions
            .delimited_by(just(Token::LCurly), just(Token::RCurly))
            .map_with_span(|block, span| (Expr::Block(block), span));

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(block.clone())
                .then(
                    just(Token::Else)
                        .ignore_then(block.clone().or(if_))
                        .or_not(),
                )
                .map_with_span(|((cond, a), b), span: Span| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            Box::new(match b {
                                Some(b) => b,
                                // If an `if` expression has no trailing `else` block, we magic up one that just produces 'nothing'.
                                None => (Expr::Value(Value::Nothing), span.clone()),
                            }),
                        ),
                        span,
                    )
                })
        });

        block.or(if_).or(sum)
    });

    expr.clone()
        .separated_by(separator)
        .allow_trailing()
        .allow_leading()
        .then_ignore(end())
        .map_with_span(|program, span| (Expr::Program(program), span))
}
