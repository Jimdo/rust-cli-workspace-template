use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum AppError {
    #[error("Unknown Error Occurred: {0}")]
    #[diagnostic(help("{1}"))]
    Unknown(String, String), // (Error String/Trace, Help Message)
}
