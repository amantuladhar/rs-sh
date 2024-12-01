use thiserror::Error;

use crate::string_parser::StringParseError;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("{0}: not found")]
    NotFound(String),

    #[error("Unknown Error")]
    UnknownError,

    #[error("{0}: error parsing argument")]
    StringParseError(#[from] StringParseError),
}
