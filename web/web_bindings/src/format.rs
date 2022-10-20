use hypatia_lib::{number::Number, units::Quantity, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub repr: String,
    pub name: String,
}

pub fn get_formats(value: &Value) -> Vec<Format> {
    [exact, approx, debug]
        .iter()
        .filter_map(|f| f(value))
        .collect()
}

fn exact(value: &Value) -> Option<Format> {
    let html = match value {
        Value::Quantity(Quantity { number, unit: _ }) => {
            if let Number::Exact(_) = number {
                Some(format!("{value}"))
            } else {
                None
            }
        }
        Value::Nothing => Some(format!("Nothing")),
        Value::Bool(b) => Some(format!("{b}")),
    };

    html.map(|html| Format {
        repr: html,
        name: "Exact".to_string(),
    })
}

fn approx(value: &Value) -> Option<Format> {
    if let Value::Quantity(Quantity { number, unit }) = value {
        Some(Format {
            repr: format!("Approx. {} {unit}", number.clone().into_approx()),
            name: "Approx".to_string(),
        })
    } else {
        None
    }
}

fn debug(value: &Value) -> Option<Format> {
    Some(Format {
        repr: format!("{value:?}"),
        name: "Debug".to_string(),
    })
}
