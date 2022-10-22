mod format;
mod utils;

use cfg_if::cfg_if;
use format::{get_formats, Format};
use hypatia_lib::{eval, parse, report_error, Environment, Error};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Duration;
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
    runtime: Option<Duration>,
    output: Result<Vec<Format>, Vec<Error>>,
}

lazy_static! {
    static ref EMPTY_ENV: Environment = Environment::new();
}

static STATE: Mutex<Vec<Cell>> = Mutex::new(Vec::new());

/// Re-run the code for a cell
fn refresh(cell_index: usize, cells: &mut Vec<Cell>) {
    let mut env = if cell_index == 0 {
        EMPTY_ENV.clone()
    } else {
        cells[cell_index - 1].environment.clone()
    };

    let cell = &mut cells[cell_index];

    let (output, runtime) = run(&cell.source_code, &mut env);
    cell.output = output;
    cell.runtime = Some(runtime);
    cell.environment = env;
}

#[wasm_bindgen]
pub fn clear_state() {
    let mut cells = STATE.lock().unwrap();
    cells.clear();
}

#[wasm_bindgen]
pub fn write_cell(cell_index: usize, code: &str) -> Vec<usize> {
    let mut cells = STATE.lock().unwrap();

    // Get the environment produced by the previous cell or use a empty env if this is the first one
    let mut env = if cell_index == 0 {
        EMPTY_ENV.clone()
    } else {
        cells.get(cell_index - 1).unwrap().environment.clone()
    };

    let cell = cells.get_mut(cell_index).expect("Invalid cell index");

    // Update the current cell
    let (output, runtime) = run(code, &mut env);
    *cell = Cell {
        source_code: code.to_string(),
        output,
        runtime: Some(runtime),
        environment: env,
    };

    // "Refresh" all of the cells dependent on the one that has changed
    // FIXME: would be nice to check beforehand if we
    // actually need to do this if the computations are heavy
    // might also be nice to not do this here but instead just return the list
    // and let the notebook choose when to update by calling just write_cell and read_cell
    // on all of the dependent cells.
    let mut refreshed_cells = vec![cell_index];

    for dep_index in (cell_index + 1)..cells.len() {
        refresh(dep_index, &mut cells);
        refreshed_cells.push(dep_index);
    }

    refreshed_cells
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
            output: Ok(Vec::new()),
            runtime: None,
        },
    );
}

#[wasm_bindgen]
pub fn remove_cell(cell_index: usize) {
    let mut cells = STATE.lock().unwrap();
    cells.remove(cell_index);

    // Refresh all of the dependent cells
    (cell_index..cells.len()).for_each(|i| refresh(i, &mut cells));
}

#[wasm_bindgen]
pub fn read_cell(cell_index: usize) -> String {
    let cells = STATE.lock().unwrap();
    let cell = cells.get(cell_index).expect("Invalid cell index");

    // This crate includes a notion of Formats which offer different
    // ways of representing a Value. To send all of the representations
    // over to the frontend a single string with "%%%" used as separator
    // The name and value is seperated from each other by "###"
    match &cell.output {
        Ok(result) => result
            .iter()
            .cloned()
            .map(|Format { repr, name }| format!("{repr}###{name}%%%"))
            .collect(),

        Err(errors) => errors
            .iter()
            .map(|e| report_error(e.clone(), &cell.source_code))
            .collect(),
    }
}

#[wasm_bindgen]
pub fn read_cell_time(cell_index: usize) -> Option<String> {
    let cells = STATE.lock().unwrap();
    let cell = cells.get(cell_index).expect("Invalid cell index");

    cell.runtime.map(|time| format!("{} ms", time.as_millis()))
}

fn run(code: &str, env: &mut Environment) -> (Result<Vec<Format>, Vec<Error>>, Duration) {
    let start_time = wasm_timer::Instant::now();
    let ast = parse(code);

    if let Err(errors) = ast {
        return (Err(errors), start_time.elapsed());
    }

    let value = eval(&ast.unwrap(), env);

    if let Err(error) = value {
        return (Err(vec![error]), start_time.elapsed());
    }

    (Ok(get_formats(&value.unwrap())), start_time.elapsed())
}
