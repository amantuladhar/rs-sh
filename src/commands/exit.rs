use super::{Command, CommandError};

pub(crate) fn exit_cmd(code: i32) {
    std::process::exit(code);
}

pub(crate) fn parse_exit_cmd(args: Vec<String>) -> Result<Command, CommandError> {
    let value = match args.is_empty() {
        true => 0,
        false => args[0].parse::<i32>().unwrap_or(1),
    };
    Ok(Command::Exit(value))
}
