use error::CommandError;

pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;

pub enum BuiltInCommand {
    Noop,
    Echo(Vec<String>),
    Exit(i32),
}

impl BuiltInCommand {
    pub fn execute(&self) {
        match self {
            BuiltInCommand::Noop => (),
            BuiltInCommand::Echo(args) => echo::echo_cmd(args),
            BuiltInCommand::Exit(code) => exit::exit_cmd(*code),
        }
    }

    pub fn from(input: &str) -> Result<Self, CommandError> {
        let mut input = input.trim().split(" ").into_iter();
        let command = input.next().expect("No command provided");
        match command {
            "" => Ok(BuiltInCommand::Noop),
            "echo" => echo::parse_echo_cmd(&mut input),
            "exit" => exit::parse_exit_cmd(&mut input),
            _ => Err(CommandError::NotFound(command.to_string())),
        }
    }
}
