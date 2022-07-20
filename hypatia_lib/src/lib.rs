mod error;
mod eval;
mod parser;
mod units;
pub use error::Error;
pub use eval::{eval_expr, Environment};
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::{Parser, Stream};

    #[test]
    fn test() {
        let src = include_str!("../tests/simple.hyp");
        let (tokens, mut errors) = parser::lexer().parse_recovery(src);
        println!("tokens: {tokens:?},\n errors {errors:?}");

        if let Some(tokens) = tokens {
            let len = src.chars().count();
            let (ast, parse_errors) =
                parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));
            println!("ast: {ast:?},\n parser errors: {parse_errors:?}");
            let mut env = Environment::new();
            println!("{:?}", eval_expr(&ast.unwrap(), &mut env));
        }
    }
}
