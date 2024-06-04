use std::str::Split;

use super::{Command, CommandError};

pub(crate) fn exit_cmd(code: i32) {
    std::process::exit(code);
}

pub(crate) fn parse_exit_cmd<'a>(input: &mut Split<&str>) -> Result<Command, CommandError> {
    let value = input.next().unwrap_or("0");
    let value = value
        .parse::<i32>()
        .map_err(|_| CommandError::Uknown(format!("Unable to parse exit code - {value}")))?;
    Ok(Command::Exit(value))
}
