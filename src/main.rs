#[allow(unused_imports)]
use std::io::{self, Write};

use commands::Command;

mod commands;
mod utils;

fn main() {
    utils::logger::setup().expect("Unable to setup logger");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match Command::from(input.trim()) {
            Ok(cmd) => cmd.execute(),
            Err(err) => {
                io::stderr()
                    .write_all(format!("{}\n", err).as_bytes())
                    .expect("Failed to write to stderr");
                io::stderr().flush().expect("Failed to flush stdout");
            }
        }
    }
}
