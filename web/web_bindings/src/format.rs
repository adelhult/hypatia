use hypatia_lib::{
    format_unit,
    number::Number,
    units::Quantity,
    Environment, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub repr: String,
    pub name: String,
}

pub fn get_formats(value: &Value, env: &Environment) -> Vec<Format> {
    [exact, approx, debug]
        .iter()
        .filter_map(|f| f(value, env))
        .collect()
}

fn exact(value: &Value, env: &Environment) -> Option<Format> {
    let html = match value {
        Value::Quantity(q @ Quantity { number, unit: _ }) => {
            if let Number::Exact(_) = number {
                let (Quantity{number: rescaled_number, unit: _}, (long_unit_str, _)) = format_unit(q.clone(), env);
                Some(format!(
                        "{rescaled_number} {long_unit_str}"
                ))
            } else {
                None
            }
        }
        Value::Nothing => Some(format!("Nothing")),
        Value::Bool(b) => Some(format!("{b}")),
        Value::Function(_) => Some(format!("Function")),
    };

    html.map(|html| Format {
        repr: html,
        name: "Exact".to_string(),
    })
}

fn approx(value: &Value, env: &Environment) -> Option<Format> {
    let Value::Quantity(q) = value else {
        return None;
    };
    let (Quantity { number, unit: _ }, (long_unit_str, _)) = format_unit(q.clone(), env);

    Some(Format {
        name: "Approx".to_string(),
        repr: format!("Approx. {} {long_unit_str}", number.into_approx()),
    })
}

fn debug(value: &Value, _: &Environment) -> Option<Format> {
    Some(Format {
        repr: format!("{value:#?}"),
        name: "Debug".to_string(),
    })
}
