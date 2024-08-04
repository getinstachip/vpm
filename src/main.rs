mod command_handler;
mod errors;
mod http;
mod installer;

use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let parse_result = command_handler::handle_args(env::args()).await;
    match parse_result {
        Err(error) => println!("Failed to parse command: {}", error),
        Ok(_) => (),
    }
}
