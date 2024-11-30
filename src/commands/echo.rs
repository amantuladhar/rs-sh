use super::{Command, CommandError};

pub(crate) fn echo_cmd(args: &Vec<String>) {
    let output = args.join(" ");
    println!("{}", output)
}

pub(crate) fn parse_echo_cmd(args: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::Echo(args))
}
