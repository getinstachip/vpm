use async_trait::async_trait;
use clap::{Parser, Subcommand};

use crate::errors::{CommandError, ParseError};
use crate::installer::Installer;
use crate::remover::Remover;
use crate::updater::Updater;
use crate::locator::Locator;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a package from a repository
    Install {
        /// Repository name <author/repo>
        repo: Option<String>,
        /// Install flex packages
        #[arg(long)]
        flex: bool,
        /// List installed packages
        #[arg(long)]
        list: bool,
    },
    /// Remove a package
    Remove {
        /// Package name
        package_name: Option<String>,
        /// List installed packages
        #[arg(long)]
        list: bool,
    },
    /// Update a package
    Update {
        /// Package name
        package_name: Option<String>,
        /// Install flex packages
        #[arg(long)]
        flex: bool,
        /// List packages that can be updated
        #[arg(long)]
        list: bool,
    },
    /// Locate a package
    Locate {
        /// Query
        query: String,
        /// Repository
        place_to_look: String,
        #[arg(long)]
        repo: bool,
        #[arg(long)]
        collection: bool,
    },
}

#[async_trait]
pub trait CommandHandler {
    async fn execute(&self) -> Result<(), CommandError>;
    async fn list() -> Result<(), ParseError>;
}

pub async fn handle_args(args: Args) -> Result<(), ParseError> {
    match args.command {
        Some(Commands::Install { repo, flex, list }) => {
            if list {
                Installer::list().await?;
                Ok(())
            } else {
                match repo {
                    Some(repo_name) => {
                        let install_handler = Installer::new(repo_name, flex);
                        install_handler.execute().await
                            .map_err(|e| ParseError::MissingArgument(e.to_string()))
                    },
                    None => Err(ParseError::MissingArgument("Repository name".to_string()))
                }
            }
        }
        Some(Commands::Remove { package_name, list }) => {
            if list {
                Remover::list().await?;
                Ok(())
            } else {
                match package_name {
                    Some(package_name) => {
                        let remover = Remover::new(package_name);
                        match remover.execute().await {
                            Ok(_) => Ok(()),
                            Err(e) => Err(ParseError::MissingArgument(e.to_string())),
                        }
                    }
                    None => Err(ParseError::MissingArgument("Package name".to_string())),
                }
            }
        }
        Some(Commands::Update { package_name, flex, list }) => {
            if list {
                Updater::list().await?;
                Ok(())
            } else {
                let updater = Updater::new(package_name, flex);
                match updater.execute().await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ParseError::MissingArgument(e.to_string())),
                }
            }
        }
        Some(Commands::Locate { query, place_to_look, repo, collection }) => {
            let locator = Locator::new(query, place_to_look, repo, collection);
            match locator.execute().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ParseError::MissingArgument(e.to_string())),
            }
        }
        _ => Err(ParseError::MissingArgument("Command not found".to_string())),
    }
}
