use async_trait::async_trait;
use std::env::Args;

use crate::{
    errors::{
        CommandError,
        ParseError::{self, CommandNotFound},
    },
    installer::Installer,
};

#[async_trait]
pub trait CommandHandler {
    fn parse(&mut self, args: &mut Args) -> Result<(), ParseError>;
    async fn execute(&self) -> Result<(), CommandError>;
}

pub async fn handle_args(mut args: Args) -> Result<(), ParseError> {
    args.next();

    let command = match args.next() {
        Some(command) => command,
        None => {
            // TODO: Implement help menu
            println!("No help menu implemented yet.");
            return Ok(());
        }
    };

    let mut command_handler: Box<dyn CommandHandler> = match command.to_lowercase().as_str() {
        "install" => Box::new(Installer::default()),
        _ => return Err(CommandNotFound(command.to_string())),
    };

    command_handler.parse(&mut args)?;
    let command_result = command_handler.execute().await;

    if let Err(error) = command_result {
        println!("Command error: {error}");
    }

    Ok(())
}

