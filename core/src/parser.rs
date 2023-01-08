use crate::expr::*;
use crate::Error;
use chumsky::{prelude::*, Stream};
use std::fmt;

// Greatly inspired by the  Chumsky tutorial
// and the Tao language implementation (https://github.com/zesterer/tao)

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
    DecimalNum(String),
    BinaryNum(String),
    HexNum(String),
    ScientificNum(String, String, bool),
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
    In,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "{}", x),
            Token::DecimalNum(x) => write!(f, "{}", x),
            Token::BinaryNum(x) => write!(f, "{}", x),
            Token::ScientificNum(base, exponent, neg_sign) => {
                if *neg_sign {
                    write!(f, "{base}E-{exponent}")
                } else {
                    write!(f, "{base}E{exponent}")
                }
            }
            Token::HexNum(x) => write!(f, "{}", x),
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
            Token::In => write!(f, "in"),
        }
    }
}

fn ident() -> impl Parser<char, Vec<char>, Error = Simple<char>> + Copy + Clone {
    filter(|c: &char| c.is_alphabetic() || *c == '_')
        .map(Some)
        .chain(filter(|c: &char| c.is_alphanumeric() || *c == '_').repeated())
}

fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    // parse number
    let frac = just('.').chain(text::digits(10));

    // 13(.37) or .32
    let decimal_form = text::int(10)
        .chain::<char, _, _>(frac.or_not().flatten())
        .or(frac)
        .collect::<String>();

    // Base 10 numbers The "or frac" part is to allow for .25 as well
    let decimal = decimal_form.map(Token::DecimalNum);

    // binary literals 0b1010
    let binary = just("0b")
        .ignore_then(text::int::<_, Simple<char>>(2))
        .map(Token::BinaryNum);

    // hexadecimal literals 0xff
    let hex = just("0x")
        .ignore_then(text::int::<_, Simple<char>>(16))
        .map(Token::HexNum);

    // scientific form 1.5e-2
    let e = just('e').or(just('E'));

    let scientific = decimal_form
        .then_ignore(e)
        .then(just('-').or_not())
        .then(text::int::<_, Simple<char>>(10))
        .map(|((base, sign), exponent)| Token::ScientificNum(base, exponent, sign.is_some()));

    // operators
    let single_char_op = select! {
        '=' => Token::Assignment,
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        '<' => Token::Lt,
        '>' => Token::Gt,
    };

    let ops = just("<=")
        .to(Token::Lte)
        .or(just(">=").to(Token::Gte))
        .or(just("==").to(Token::Equal))
        .or(just("!=").to(Token::NotEqual))
        .or(single_char_op);

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

    let keywords_and_idents = ident().map(|v| match String::from_iter(v).as_str() {
        "unit" => Token::Unit,
        "not" => Token::Not,
        "prefix" => Token::Prefix,
        "if" => Token::If,
        "else" => Token::Else,
        "update" => Token::Update,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "nothing" => Token::Nothing,
        "in" => Token::In,
        s => Token::Ident(s.into()),
    });

    let comment = just("//").then(take_until(just('\n'))).to(Token::Comment);

    let token = comment
        .or(binary)
        .or(hex)
        .or(scientific)
        .or(decimal)
        .or(control)
        .or(ops)
        .or(keywords_and_idents)
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
            Token::DecimalNum(n) => NumberLiteral::Decimal(n),
            Token::BinaryNum(n) => NumberLiteral::Binary(n),
            Token::HexNum(n) => NumberLiteral::Hex(n),
            Token::ScientificNum(base, exp, neg_sign) => NumberLiteral::Scientific(base, exp, neg_sign),
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

        let parameter_list = ident
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing();

        // General named function assignment syntax
        // f(x) = 10 + x
        let function = ident
            .then(
                parameter_list
                    .clone()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .then_ignore(just(Token::Assignment))
            .then(expr.clone());

        // Declare a new function
        let function_decl = function
            .clone()
            .map(|((name, params), body)| Expr::FunctionDecl(name, params, Box::new(body)));

        // A name can also be reassigned to a function
        // update f(x) = 10 + x
        let function_update = just(Token::Update)
            .ignore_then(function)
            .map(|((name, params), body)| Expr::FunctionUpdate(name, params, Box::new(body)));

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
            .or(function_update)
            .or(function_decl)
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
            .then(op.then(product.clone()).repeated())
            .foldl(|a, (operator, b)| {
                let span = a.1.start..b.1.end;
                (Expr::BinOp(operator, Box::new(a), Box::new(b)), span)
            });

        // Comparison operators
        let op = just(Token::Lt)
            .to(BinOp::Lt)
            .or(just(Token::Lte).to(BinOp::Lte))
            .or(just(Token::Gt).to(BinOp::Gt))
            .or(just(Token::Gte).to(BinOp::Gte))
            .or(just(Token::Equal).to(BinOp::Equal))
            .or(just(Token::NotEqual).to(BinOp::NotEqual));

        let comparison = sum
            .clone()
            .then(op.then(sum).repeated())
            .foldl(|a, (operator, b)| {
                let span = a.1.start..b.1.end;
                (Expr::BinOp(operator, Box::new(a), Box::new(b)), span)
            });

        // FIXME: logic operators

        // 20 m + 3 km in miles
        let conversion = comparison
            .clone()
            .then_ignore(just(Token::In))
            .then(product)
            .map_with_span(|(e, unit), span| (Expr::Conversion(Box::new(e), Box::new(unit)), span));

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

        block.or(if_).or(conversion).or(comparison)
    });

    expr.clone()
        .separated_by(separator)
        .allow_trailing()
        .allow_leading()
        .then_ignore(end())
        .map_with_span(|program, span| (Expr::Program(program), span))
}
