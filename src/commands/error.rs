use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("{0}: command not found")]
    NotFound(String),
}
