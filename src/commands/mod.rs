pub use error::CommandError;
use exit::parse_exit_cmd;

pub(crate) mod error;
pub(crate) mod exit;

pub enum Command {
    Noop,
    Exit(i32),
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::Noop => (),
            Command::Exit(code) => exit::exit_cmd(*code),
        }
    }

    pub fn from(input: &str) -> Result<Self, CommandError> {
        let mut input = input.trim().split(" ").into_iter();
        let command = input.next().expect("No command provided");
        match command {
            "" => Ok(Command::Noop),
            "exit" => parse_exit_cmd(&mut input),
            _ => Err(CommandError::NotFound(command.to_string())),
        }
    }
}
