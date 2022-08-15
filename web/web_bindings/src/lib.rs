mod utils;

use cfg_if::cfg_if;
use hypatia_lib::{eval, parse, report_error, Environment, Error};
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn evaluate(src: &str) -> String {
    let mut env = Environment::default();
    match run(src, &mut env) {
        Err(errors) => errors.into_iter().map(|e| report_error(e, src)).collect(),
        Ok(result) => result,
    }
}

fn run(source: &str, env: &mut Environment) -> Result<String, Vec<Error>> {
    let ast = parse(source)?;
    let value = eval(&ast, env).map_err(|error| vec![error])?;
    Ok(format!("{value}"))
}
