use async_trait::async_trait;
use std::env::Args;

use super::install::InstallHandler;

#[async_trait]
pub trait CommandHandler {
    fn parse(&mut self, args: &mut Args) -> ();
    async fn execute(&self) -> ();
}

pub async fn handle_args(mut args: Args) -> () {
    args.next();

    let command = match args.next() {
        Some(command) => command,
        None => {
            // TODO: Implement help menu
            println!("No help menu implemented yet.");
            return;
        }
    };

    let mut command_handler: Box<dyn CommandHandler> = match command.to_lowercase().as_str() {
        "install" => Box::<InstallHandler>::default(),
        _ => return
    };
}
