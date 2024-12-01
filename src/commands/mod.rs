use std::str::FromStr;

use error::CommandError;

use crate::string_parser::StringParser;

pub(crate) mod cd;
pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod external;
pub(crate) mod pwd;
pub(crate) mod typee;

pub enum Command {
    Noop,
    Echo(Vec<String>),
    Exit(i32),
    Type(String),
    Pwd,
    Cd(String),
    External {
        #[allow(dead_code)]
        cmd: String,
        args: Vec<String>,
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
            Cd(args) => cd::cd_cmd(args),
            External { .. } => external::external_cmd(self),
        }
    }
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let input = {
            let mut parser = StringParser::new(input);
            match parser.get_processed_args() {
                Err(e) => return Err(CommandError::StringParseError(e)),
                Ok(args) => args,
            }
        };
        let cmd = input.get(0).ok_or_else(|| CommandError::UnknownError)?;
        let args = if input.len() > 1 {
            input[1..].to_vec()
        } else {
            vec![]
        };

        Ok(match cmd.as_str() {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(args)?,
            "exit" => exit::parse_exit_cmd(args)?,
            "type" => typee::parse_type_cmd(args)?,
            "pwd" => pwd::parse_pwd_cmd()?,
            "cd" => cd::parse_cd_cmd(args)?,
            _ => match external::parse_external_cmd(cmd, args) {
                Some(cmd) => cmd,
                None => return Err(CommandError::NotFound(cmd.to_string())),
            },
        })
    }
}
