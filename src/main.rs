mod errors;
mod http;
mod installer;
mod embedding;

use clap::{Parser, Subcommand}; // Ensure you import Parser if you're using it for Args
use errors::ParseError; // Import ParseError for error handling
use async_trait::async_trait;

use crate::errors::{
    CommandError,
    ParseError::CommandNotFound,
};

use crate::installer::Installer;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>, // Ensure this matches your command structure
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Install {
        /// The author/repo to install
        repo: String,
        /// Optional flag for flexible installation
        #[arg(long)]
        flex: bool,
    },
}

/// Trait for command handlers
#[async_trait]
pub trait CommandHandler {
    async fn execute(&self) -> Result<(), CommandError>;
}

#[tokio::main]
async fn main() {
    let args = Args::parse(); // Parse command-line arguments
    let result = handle_args(args).await; // Call handle_args asynchronously

    if let Err(e) = result {
        eprintln!("Error: {}", e); // Print error if it occurs
    }
}

pub async fn handle_args(args: Args) -> Result<(), ParseError> {
    match args.command {
        Some(Commands::Install { repo, flex }) => {
            let install_handler = Installer::new(repo, flex);
            match install_handler.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string()))
            }
        }
        None => Err(CommandNotFound(String::from("")))
    }
}
