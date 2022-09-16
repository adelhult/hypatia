use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, prelude::Simple};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub enum Error {
    Parsing(Simple<String>),
    ErrorNode,
    UnknownName(String),
    UpdateNonExistentVar(String),
    InvalidType,
    InvalidUnitOperation,
    OccupiedName(String),
}

pub fn report_error(error: Error, src: &str) -> String {
    match error {
        Error::Parsing(error) => {
            let mut result = Cursor::new(Vec::new());
            let report = Report::build(ReportKind::Error, (), error.span().start);
            let report = match error.reason() {
                SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(error.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                error
                                    .found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if error.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if error.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            error
                                .expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(error.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                error
                                    .found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(error.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };
            report
                .finish()
                .write(Source::from(src), &mut result)
                .unwrap();

            String::from_utf8(result.into_inner()).unwrap()
        }
        // FIXME: add spans to these, then we can create nicer
        //  error reports for these as well
        Error::ErrorNode => String::from("Error node"),
        Error::UnknownName(name) => format!("Unknown name {name}."),
        Error::UpdateNonExistentVar(name) => {
            format!("You cannot update the variable {name} because it has not been declared yet.")
        }
        Error::InvalidType => "Invalid type.".to_string(),
        Error::InvalidUnitOperation => "Invalid unit operation.".to_string(),
        Error::OccupiedName(name) => format!("Occupied name {name}."),
    }
}
