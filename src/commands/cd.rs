use super::{Command, CommandError};

pub(crate) fn cd_cmd(args: &str) {
    use std::env::set_current_dir;
    match set_current_dir(args) {
        Ok(_) => (),
        Err(_) => eprintln!("cd: {}: No such file or directory", args),
    }
}

pub(crate) fn parse_cd_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Cd(args.to_owned()))
}
