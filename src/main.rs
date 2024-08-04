mod command_handler;
mod installer;
mod errors;
mod http;

use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let parse_result = command_handler::handle_args(env::args()).await;
    match parse_result {
        Err(error) => println!("Failed to parse command: {}", error),
        Ok(_) => (),
    }
}
