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
        repo: Option<String>,
        #[arg(long)]
        flex: bool,
        #[arg(long)]
        list: bool,
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
        Some(Commands::Install { repo, flex, list }) => {
            if list {
                list_installed_packages()?;
                Ok(())
            } else if let Some(repo) = repo {
                let install_handler = Installer::new(repo, flex);
                match install_handler.execute().await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ParseError::MissingArgument(e.to_string())),
                }
            } else {
                Err(ParseError::MissingArgument("Repository name is required for installation".to_string()))
            }
        }
        None => Err(CommandNotFound(String::from(""))),
    }
}

use std::fs;
use std::path::Path;

fn list_installed_packages() -> Result<(), ParseError> {
    let vpm_toml_path = Path::new("./vpm.toml");
    if !vpm_toml_path.exists() {
        println!("No packages installed. vpm.toml file not found.");
        return Ok(());
    }

    let vpm_toml_content = fs::read_to_string(vpm_toml_path)
        .map_err(|e| ParseError::MissingArgument(format!("Failed to read vpm.toml: {}", e)))?;

    let mut found_dependencies = false;
    for line in vpm_toml_content.lines() {
        if line.trim() == "[dependencies]" {
            found_dependencies = true;
            println!("Installed packages:");
            continue;
        }
        if found_dependencies && !line.trim().is_empty() {
            println!("  {}", line.trim());
        }
    }

    if !found_dependencies {
        println!("No packages installed.");
    }

    Ok(())
}