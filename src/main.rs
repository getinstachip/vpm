mod embedding;
mod errors;
mod http;
mod installer;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use errors::ParseError;

use crate::errors::{CommandError, ParseError::CommandNotFound};

use crate::installer::Installer;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Install {
        repo: String,
        #[arg(long)]
        flex: bool,
    },
}

#[async_trait]
pub trait CommandHandler {
    async fn execute(&self) -> Result<(), CommandError>;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let result = handle_args(args).await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

pub async fn handle_args(args: Args) -> Result<(), ParseError> {
    match args.command {
        Some(Commands::Install { repo, flex }) => {
            let install_handler = Installer::new(repo, flex);
            match install_handler.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        }
        None => Err(CommandNotFound(String::from(""))),
    }
}
