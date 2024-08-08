mod errors;
mod http;
mod installer;
mod headers;
mod remover;
mod updater;
mod command_handler;
mod includer;

use std::env;

use clap::Parser;
use command_handler::Args;

#[tokio::main]
async fn main() {
    // Forcibly disable backtraces
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    let result = command_handler::handle_args(Args::parse()).await;
    if let Err(e) = result {
        eprintln!("Failed to handle arguments. Error: {}", e);
    }
}
