use crate::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Error,
    Literal(Literal),
    Variable(String),
    VarDeclaration(String, Box<Spanned<Self>>),
    VarUpdate(String, Box<Spanned<Self>>),
    Call(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Block(Vec<Spanned<Self>>),
    Program(Vec<Spanned<Self>>),
    BinOp(BinOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
    BaseUnitDeclaration(String, Option<String>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Nothing,
    Bool(bool),
    Quantity(f64, Option<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nothing,
    Bool(bool),
    Number(f64),
}

impl Value {
    pub fn is_true(&self) -> Result<bool, Error> {
        match self {
            Value::Nothing => Ok(false),
            Value::Bool(b) => Ok(*b),
            Value::Number(_) => Err(Error::InvalidType),
        }
    }

    pub fn is_false(&self) -> Result<bool, Error> {
        Ok(!self.is_true()?)
    }

    pub fn number(&self) -> Result<f64, Error> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(Error::InvalidType),
        }
    }
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
