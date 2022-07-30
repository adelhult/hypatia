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

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);
