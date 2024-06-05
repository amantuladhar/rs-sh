use super::Command;
use super::Command::*;

pub(crate) fn parse_external_cmd(cmd: &str, args: &str) -> Option<Command> {
    let Ok(path_env) = std::env::var("PATH") else {
        return None;
    };
    path_env
        .split(':')
        .map(|path| {
            let full_path = format!("{}/{}", path, cmd);
            std::fs::metadata(&full_path).map(|_| full_path)
        })
        .find_map(Result::ok)
        .map(|path| External {
            cmd: cmd.to_string(),
            args: args.to_string(),
            path,
        })
}
