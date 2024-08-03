mod command_parser;
mod installer;

use std::env;

fn main() {
    let parse_result = command_parser::handle_args(env::args());
    match parse_result {
        Err(error) => println!("Failed to execute command: Error: {}", error),
        Ok(_) => (),
    }
}
