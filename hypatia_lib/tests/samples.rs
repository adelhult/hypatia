use hypatia_lib::{eval, parse, Environment};
use std::{
    fs,
    path::{Path, PathBuf},
};

// TODO: Add support for testing error outputs as well
fn run_test_file(source_file: &Path) {
    let file = fs::read_to_string(source_file).expect("Failed to read the file.");

    let (source, result) = file.split_once("// Result:").expect("Bad format of sample");

    let ast = parse(source).expect("Failed to parse the source text");
    let mut env = Environment::default();
    let value = eval(&ast, &mut env).expect("Failed to evaluate the expression");
    assert_eq!(result.trim(), &format!("{value}"))
}

#[test]
fn empty() {
    run_test_file(&PathBuf::from("./samples/empty.hyp"));
}

#[test]
fn simple() {
    run_test_file(&PathBuf::from("./samples/simple.hyp"));
}
