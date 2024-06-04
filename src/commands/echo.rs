use super::{Command, CommandError};
use std::{
    io::{self, Write},
    str::Split,
};

pub(crate) fn echo_cmd(args: &Vec<String>) {
    println!("{}", args.join(" "));
    io::stdout().flush().unwrap();
}

pub(crate) fn parse_echo_cmd<'a>(input: &mut Split<&str>) -> Result<Command, CommandError> {
    let args = input
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Ok(Command::Echo(args))
}
