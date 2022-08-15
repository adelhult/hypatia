mod utils;

use cfg_if::cfg_if;
use hypatia_lib::{eval, parse, Environment, Error};
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}

#[wasm_bindgen]
pub fn evaluate(src: &str) -> String {
    let mut env = Environment::default();
    match run(src, &mut env) {
        Err(errors) => format!("{errors:?}"),
        Ok(result) => result,
    }
}

fn run(source: &str, env: &mut Environment) -> Result<String, Vec<Error>> {
    let ast = parse(source)?;
    let value = eval(&ast, env).map_err(|error| vec![error])?;
    Ok(format!("{value}"))
}
