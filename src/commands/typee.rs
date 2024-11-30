use super::{Command, CommandError};
use std::str::FromStr;

pub(crate) fn type_cmd(command: &str) {
    use Command::*;
    if command.is_empty() {
        eprint!("");
        return;
    }
    match Command::from_str(command) {
        Ok(External { path, .. }) => {
            println!("{} is {}", command, path);
        }
        Ok(_) => {
            println!("{} is a shell builtin", command);
        }
        Err(e) => {
            eprintln!("{}", e.to_string())
        }
    };
}

pub(crate) fn parse_type_cmd(args: Vec<String>) -> Result<Command, CommandError> {
    let args = if args.is_empty() {
        ""
    } else {
        args[0].trim()
    };
    Ok(Command::Type(args.to_owned()))
}
