use super::{Command, CommandError};

pub(crate) fn cd_cmd(args: &str) {
    use std::env::{set_current_dir, var};
    let args = match args {
        "~" => var("HOME").unwrap_or("/".into()),
        _ => args.into(),
    };
    match set_current_dir(&args) {
        Ok(_) => (),
        Err(_) => eprintln!("cd: {}: No such file or directory", &args),
    }
}

pub(crate) fn parse_cd_cmd(args: Vec<String>) -> Result<Command, CommandError> {
    let arg = if args.is_empty() {
        "".to_string()
    } else {
        args[0].clone()
    };
    Ok(Command::Cd(arg))
}
