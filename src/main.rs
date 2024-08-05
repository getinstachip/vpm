mod embedding;
mod errors;
mod http;
mod installer;
mod headers;
mod remover;
mod updater;
mod command_handler;

use clap::Parser;
use command_handler::Args;

#[tokio::main]
async fn main() {
    let result = command_handler::handle_args(Args::parse()).await;
    if let Err(e) = result {
        eprintln!("Failed to handle arguments: {}", e);
    }
}