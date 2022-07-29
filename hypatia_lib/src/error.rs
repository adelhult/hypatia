use chumsky::prelude::Simple;

#[derive(Debug)]
pub enum Error {
    Parsing(Simple<String>),
    ErrorNode,
    UnknownName(String),
    UpdateNonExistentVar(String),
    InvalidType,
}
