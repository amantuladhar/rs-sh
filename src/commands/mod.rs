use error::CommandError;

pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod external;
pub(crate) mod pwd;
pub(crate) mod typee;

pub enum Command {
    Noop,
    Echo(String),
    Exit(i32),
    Type(String),
    Pwd,
    External {
        cmd: String,
        args: String,
        path: String,
    },
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            Noop => (),
            Echo(args) => echo::echo_cmd(args),
            Exit(code) => exit::exit_cmd(*code),
            Type(args) => typee::type_cmd(args),
            Pwd => pwd::pwd_cmd(),
            External { .. } => external::external_cmd(self),
        }
    }

    pub fn from(input: &str) -> Result<Self, CommandError> {
        use Command::*;
        let input = input.trim().splitn(2, ' ').collect::<Vec<&str>>();
        let cmd = input.get(0).copied().unwrap_or("");
        let args = input.get(1).copied().unwrap_or("");
        Ok(match cmd {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(args)?,
            "exit" => exit::parse_exit_cmd(args)?,
            "type" => typee::parse_type_cmd(args)?,
            "pwd" => Command::Pwd,
            _ => match external::parse_external_cmd(cmd, args) {
                Some(cmd) => cmd,
                None => return Err(CommandError::NotFound(cmd.to_string())),
            },
        })
    }
}
