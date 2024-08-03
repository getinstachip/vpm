use std::env::Args;

use crate::installer::Installer;

pub trait CommandHandler {
    fn parse(&mut self, args: &mut Args);
    fn execute(&self);
}

pub fn handle_args(mut args: Args) {
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
        "install" => Box::new(Installer::default()),
        _ => return
    };

    command_handler.parse(&mut args);
    command_handler.execute();
}
