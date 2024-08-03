mod command_parser;
mod installer;

use std::env;

fn main() {
    command_parser::handle_args(env::args());
}
