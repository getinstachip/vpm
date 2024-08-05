use clap::Parser;

mod embedding;
mod errors;
mod http;
mod installer;
mod headers;
mod remover;
mod updater;
mod command_handler;

#[tokio::main]
async fn main() {
    let args = command_handler::Args::parse();
    let result = command_handler::handle_args(args).await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}