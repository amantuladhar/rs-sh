use super::{Command, CommandError};

pub(crate) fn type_cmd(command: &str) {
    use Command::*;
    if command.is_empty() {
        eprint!("");
        return;
    }
    match Command::from(command) {
        Ok(Exit(..) | Echo(..) | Type(..) | Pwd | Noop | Cd(..)) => {
            println!("{} is a shell builtin", command);
        }
        Ok(External { path, .. }) => {
            println!("{} is {}", command, path);
        }
        Err(CommandError::NotFound(..)) => {
            eprintln!("{} not found", command)
        }
    };
}

pub(crate) fn parse_type_cmd(args: &str) -> Result<Command, CommandError> {
    let args = args.trim();
    Ok(Command::Type(args.to_owned()))
}
