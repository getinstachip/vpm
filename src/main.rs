mod commands;
mod installer;
mod util;

use std::env;

use commands::command_handler;

#[tokio::main]
async fn main() {
    let parse_result = command_handler::handle_args(env::args()).await;
    if let Err(err) = parse_result {
        println!("Command does not exist, Error: {}", err);
    }
}
