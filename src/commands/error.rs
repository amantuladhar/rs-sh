use thiserror::Error;

use crate::args_parser::ArgsParseError;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("{0}: not found")]
    NotFound(String),

    #[error("{0}: error parsing argument")]
    ArgsParserError(#[from] ArgsParseError),
}
