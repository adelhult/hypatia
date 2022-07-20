#[derive(Debug)]
pub enum Error {
    Parser,
    UnknownName(String),
    InvalidType,
}
