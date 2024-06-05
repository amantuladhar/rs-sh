use error::CommandError;

pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod typee;

pub enum Command {
    Noop,
    Echo(String),
    Exit(i32),
    Type(String),
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            Noop => (),
            Echo(args) => echo::echo_cmd(args),
            Exit(code) => exit::exit_cmd(*code),
            Type(args) => typee::type_cmd(args),
        }
    }

    pub fn from(input: &str) -> Result<Self, CommandError> {
        use Command::*;
        let input = input.trim().splitn(2, ' ').collect::<Vec<&str>>();
        let command = input.get(0).copied().unwrap_or("");
        let args = input.get(1).copied().unwrap_or("");
        let command = match command {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(args)?,
            "exit" => exit::parse_exit_cmd(args)?,
            "type" => typee::parse_type_cmd(args)?,
            _ => return Err(CommandError::NotFound(command.to_string())),
        };
        Ok(command)
    }
}
