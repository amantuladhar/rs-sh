#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

use commands::Command;

mod commands;
mod string_parser;
mod utils;

fn main() {
    utils::logger::setup().expect("Unable to setup logger");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match Command::from_str(input.trim()) {
            Ok(cmd) => cmd.execute(),
            Err(err) => eprintln!("{err}"),
        };

        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}
