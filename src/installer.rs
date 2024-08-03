use std::env::Args;

use crate::command_parser::CommandHandler;

#[derive(Debug, Default)]
pub struct Installer {
    package_author: String,
    package_name: String,
}

impl Installer {
    fn parse_package_details(package_details: String) -> (String, String) {
        let mut split = package_details.split("/");

        let author = split
            .next()
            .expect("Provided package author is empty")
            .to_string();

        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        (author, name)
    }
}

impl CommandHandler for Installer {
    fn parse(&mut self, args: &mut Args) {
        let package_details = match args.next() {
            Some(package_details) => package_details,
            None => return,
        };

        let (package_author, package_name) = Self::parse_package_details(package_details)?;
        self.package_name = package_name;
        self.package_author = package_author;
    }

    fn execute(&self) {
        println!("Installing {}", self.package_name);
    }
}
