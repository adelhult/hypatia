/*!
This crates contains everything needed to parse and evaluate the Hypatia language.

# Getting started example
```
use hypatia_lib::{parse, eval, Value, Environment};
let source = "40 + 2";
let ast = parse(&source).expect("Failed to parse source text");
let mut env = Environment::default();
let value = eval(&ast, &mut env).expect("Failed to evaluate the expression");
assert_eq!(value.to_string(), "42".to_string());
```
*/
mod error;
mod eval;
pub mod number;

mod resolve;
#[allow(dead_code)]
mod trie;
pub mod units;

pub use error::{report_error, Error};
pub use eval::*;
pub use syntax::expr::{Expr, Spanned};
use syntax::parser;

pub fn parse(source: &str) -> Result<Spanned<Expr>, Vec<Error>> {
    let expr = parser::parse(source).map_err(|errors| {
        errors
            .into_iter()
            .map(Error::Parsing)
            .collect::<Vec<Error>>()
    })?;

    resolve::resolve(expr).map_err(|error| vec![error])
}
