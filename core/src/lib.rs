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
mod expr;
mod parser;
mod number;
#[allow(dead_code)]
mod trie;
mod units;

pub use error::{report_error, Error};
pub use eval::*;
pub use expr::Expr;
pub use parser::parse;
