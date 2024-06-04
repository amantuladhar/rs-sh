#[allow(unused_imports)]
use std::io::{self, Write};

mod utils;

fn main() {
    utils::logger::setup().expect("Unable to setup logger");

    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
}
