use std::env::Args;

use crate::{
    command_parser::CommandHandler,
    errors::{
        CommandError,
        ParseError::{self, MissingArgument},
    }
};

#[derive(Debug, Default)]
pub struct Installer {
    package_author: String,
    package_name: String,
}

impl Installer {
    fn parse_package_details(package_details: String) -> Result<(String, String), ParseError> {
        let mut split = package_details.split('/');

        let author = split
            .next()
            .expect("Provided package author is empty")
            .to_string();

        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        Ok((author, name))
    }
}

impl CommandHandler for Installer {
    fn parse(&mut self, args: &mut Args) -> Result<(), ParseError> {
        let package_details = match args.next() {
            Some(package_details) => package_details,
            None => return Err(MissingArgument("package name".to_string())),
        };

        let (package_author, package_name) = Self::parse_package_details(package_details)?;
        self.package_name = package_name;
        self.package_author = package_author;

        Ok(())
    }

    fn execute(&self) -> Result<(), CommandError> {
        println!("Installing {}", self.package_name);
        Ok(())
    }
}
