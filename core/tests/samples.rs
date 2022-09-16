use hypatia_lib::{eval, parse, Environment};
use std::{fs, path::Path};

// TODO: Add support for testing error outputs as well
fn run_test_file(source_file: &Path) {
    let file = fs::read_to_string(source_file).expect("Failed to read the file.");

    let (source, result) = file.split_once("// Result:").expect("Bad format of sample");

    let ast = parse(source).expect("Failed to parse the source text");
    let mut env = Environment::default();
    let value = eval(&ast, &mut env).expect("Failed to evaluate the expression");
    assert_eq!(result.trim(), &format!("{value}"));
}

#[test]
fn empty() {
    run_test_file(Path::new("./samples/empty.hyp"));
}

#[test]
fn simple() {
    run_test_file(Path::new("./samples/simple.hyp"));
}

#[test]
fn if_else() {
    run_test_file(Path::new("./samples/if_else.hyp"));
}

#[test]
fn if_nothing() {
    run_test_file(Path::new("./samples/if_nothing.hyp"));
}

#[test]
fn scopes() {
    run_test_file(Path::new("./samples/scopes.hyp"));
}

#[test]
fn update() {
    run_test_file(Path::new("./samples/update.hyp"));
}

#[test]
fn base_units() {
    run_test_file(Path::new("./samples/base_units.hyp"));
}

#[test]
fn unit_arithmetic() {
    run_test_file(Path::new("./samples/unit_arithmetic.hyp"));
}

#[test]
fn derived_units() {
    run_test_file(Path::new("./samples/derived_units.hyp"));
}

#[test]
fn unit_expressions() {
    run_test_file(Path::new("./samples/unit_expressions.hyp"));
}

#[test]
fn replace_units() {
    run_test_file(Path::new("./samples/replace_units.hyp"));
}

#[test]
fn declare_prefix() {
    run_test_file(Path::new("./samples/declare_prefix.hyp"));
}

#[test]
fn unary_operators() {
    run_test_file(Path::new("./samples/unary_operators.hyp"));
}

#[test]
fn unicode_ident() {
    run_test_file(Path::new("./samples/unicode_ident.hyp"));
}
