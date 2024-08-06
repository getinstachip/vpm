mod embedding;
mod errors;
mod http;
mod installer;
mod headers;
mod remover;
mod updater;
mod adder;
mod collection_creator;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use errors::ParseError;
use std::fs;
use std::path::Path;
use dotenv::dotenv;

use crate::errors::{CommandError, ParseError::CommandNotFound};

use crate::installer::Installer;
use crate::remover::Remover;
use crate::updater::Updater;
use crate::locator::Locator;
use crate::collection_creator::CollectionCreator;
use crate::adder::Adder;

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
    Remove {
        package_name: String,
    },
    Update {
        package_name: Option<String>,
        #[arg(long)]
        flex: bool,
    },
    Locate {
        query: String,
        repo: String,
    },
    Create {
        name: String,
    },
    Add {
        package_path: String,
        collection_name: String,
    },
}

#[async_trait]
pub trait CommandHandler {
    async fn execute(&self) -> Result<(), CommandError>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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
        Some(Commands::Remove { package_name }) => {
            let remover = Remover::new(package_name);
            match remover.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        },
        Some(Commands::Update { package_name, flex }) => {
            let updater = Updater::new(package_name, flex);
            match updater.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        },
        Some(Commands::Locate { query, repo }) => {
            let locator = Locator::new(query, repo);
            match locator.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        },
        Some(Commands::Create { name }) => {
            let creator = CollectionCreator::new(name);
            match creator.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        },
        Some(Commands::Add { package_path, collection_name }) => {
            let adder = Adder::new(package_path, collection_name);
            match adder.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        },
    }
}

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
