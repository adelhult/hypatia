use chumsky::prelude::*;
use std::{fmt, collections::HashMap};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),
    Number(String),
    Bool(bool),
    Unit,
    If,
    Else,
    Null,
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
            Token::Null => write!(f, "null"),
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
    }.or(text::newline().to(Token::Newline));

    // TODO: support more then just c idents
    let ident = text::ident().map(|i: String| match i.as_str() {
        "unit" => Token::Unit,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
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


// #[derive(Debug, PartialEq, Eq, Hash)]
// enum Expr {
// }

// fn parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
   
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        dbg!(lexer().parse(include_str!("../tests/example.hyp")));
    }
}
