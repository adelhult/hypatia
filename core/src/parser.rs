use crate::expr::*;
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
    Prefix,
    Not,
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
            Token::Not => write!(f, "not"),
            Token::Prefix => write!(f, "prefix"),
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
        "not" => Token::Not,
        "prefix" => Token::Prefix,
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
        let ident = select! {Token::Ident(i) => i}.labelled("identifier");

        let number = select! {
            Token::Number(n) => n.parse().expect("Could not parse number")
        };

        let quantity = number
            .then(ident.or_not())
            .map(|(number, unit)| Expr::Literal(Literal::Quantity(number, unit)));

        let value = select! {
            Token::Nothing => Expr::Literal(Literal::Nothing),
            Token::Bool(x) => Expr::Literal(Literal::Bool(x)),
        }
        .or(quantity)
        .labelled("value");

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

        // General syntax for unit declarations
        let unit_decl = just(Token::Unit).ignore_then(ident).then(ident.or_not());

        // unit meter m
        let base_unit_decl = unit_decl
            .clone()
            .map(|(long_name, short_name)| Expr::BaseUnitDecl(long_name, short_name));

        // derived units also has a right hand side
        // unit mile mi = 1609.344 m
        let derived_unit_decl = unit_decl
            .then_ignore(just(Token::Assignment))
            .then(expr.clone())
            .map(|((long_name, short_name), expr)| {
                Expr::DerivedUnitDecl(long_name, short_name, Box::new(expr))
            });

        // prefix foo f = 42
        let prefix_decl = just(Token::Prefix)
            .ignore_then(ident)
            .then(ident.or_not())
            .then_ignore(just(Token::Assignment))
            .then(expr.clone())
            .map(|((long_name, short_name), expr)| {
                Expr::PrefixDecl(long_name, short_name, Box::new(expr))
            });

        let atom = value
            .or(var_update)
            .or(var_declaration)
            .or(derived_unit_decl)
            .or(base_unit_decl)
            .or(prefix_decl)
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

        /*
            let op = just(Token::Op(Op::Sub)).to(ast::UnaryOp::Neg)
            .or(just(Token::Op(Op::Not)).to(ast::UnaryOp::Not))
            .map_with_span(SrcNode::new);
        let unary = op.repeated()
            .then(chained.labelled("unary operand"))
            .foldr(|op, expr| {
                let span = op.span().union(expr.span());
                SrcNode::new(ast::Expr::Unary(op, expr), span)
            })
            .boxed();
        */

        let op = just(Token::Sub)
            .to(UnaryOp::Negate)
            .or(just(Token::Not).to(UnaryOp::Not));

        let unary =
            op.repeated()
                .then(call.labelled("unary operand"))
                .foldr(|op, (expr, expr_span)| {
                    (
                        Expr::UnaryOp(op, Box::new((expr, expr_span.clone()))),
                        expr_span, // FIXME: this does not include the unary operator itself
                    )
                });

        // Product operators '*' and '/'
        let op = just(Token::Mul)
            .to(BinOp::Mul)
            .or(just(Token::Div).to(BinOp::Div));

        let product = unary
            .clone()
            .then(op.then(unary).repeated())
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
                                None => (Expr::Literal(Literal::Nothing), span.clone()),
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
