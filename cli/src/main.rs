use console::style;
use dialoguer::Input;
use hypatia_lib::{eval, format_unit, parse, report_error, units::Quantity, Environment, Error, Value};

fn run(source: &str, env: &mut Environment) -> Result<String, Vec<Error>> {
    let ast = parse(source)?;
    let value = eval(&ast, env).map_err(|error| vec![error])?;
    Ok(match value {
        Value::Quantity(quantity) => {
            let (Quantity{number, unit: _}, (long_name, _)) = format_unit(quantity, env);
            format!("{number} {long_name}")
        }
        other => format!("{other}"),
    })
}

fn get_input() -> Option<String> {
    let mut result = String::new();
    let mut open_blocks = 0;
    loop {
        let indent = "   ".repeat(open_blocks);
        let line: String = Input::new().with_initial_text(indent).interact().ok()?;
        result.push_str(&line);
        result.push('\n');
        // If we are not waiting for closing a curly
        open_blocks += line.matches('{').count();
        open_blocks -= line.matches('}').count();

        if open_blocks == 0 {
            break;
        }
    }
    Some(result)
}

fn main() {
    let mut env = Environment::default();
    loop {
        if let Some(input) = get_input() {
            match run(&input, &mut env) {
                Err(errors) => {
                    for error in errors {
                        println!("{}", style(report_error(error, &input)).red());
                    }
                }
                Ok(result) => println!("{}", style(result).green()),
            }
        }
    }
}
