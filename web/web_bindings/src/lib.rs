mod utils;

use cfg_if::cfg_if;
use hypatia_lib::{eval, parse, report_error, Environment, Error};
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Debug, Clone)]
struct Cell {
    environment: Environment,
    source_code: String,
    output: Result<String, Vec<Error>>,
}

lazy_static! {
    static ref EMPTY_ENV: Environment = Environment::new();
}

static STATE: Mutex<Vec<Cell>> = Mutex::new(Vec::new());

#[wasm_bindgen]
pub fn write_cell(cell_index: usize, code: &str) {
    let mut cells = STATE.lock().unwrap();

    // Get the environment produced by the previous cell or use a empty env if this is the first one
    let mut env = if cell_index == 0 {
        EMPTY_ENV.clone()
    } else {
        cells.get(cell_index - 1).unwrap().environment.clone()
    };

    let cell = cells.get_mut(cell_index).expect("Invalid cell index");
    cell.source_code = code.to_string();

    // Evaluate the code and update the stored output and env
    cell.output = run(code, &mut env);
    cell.environment = env;
}

#[wasm_bindgen]
pub fn insert_cell(cell_index: usize) {
    let mut cells = STATE.lock().unwrap();
    // Fixme: don't like this dummy state
    cells.insert(
        cell_index,
        Cell {
            environment: Environment::new(),
            source_code: String::new(),
            output: Ok(String::new()),
        },
    );
}

#[wasm_bindgen]
pub fn read_cell(cell_index: usize) -> String {
    let cells = STATE.lock().unwrap();
    let cell = cells.get(cell_index).expect("Invalid cell index");

    match &cell.output {
        Ok(result) => result.clone(),
        Err(errors) => errors
            .iter()
            .map(|e| report_error(e.clone(), &cell.source_code))
            .collect(),
    }
}

fn run(code: &str, env: &mut Environment) -> Result<String, Vec<Error>> {
    let ast = parse(code)?;
    let value = eval(&ast, env).map_err(|error| vec![error])?;
    Ok(format!("{value}"))
}
