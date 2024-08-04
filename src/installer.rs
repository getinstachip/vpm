use async_trait::async_trait;
use std::{env::Args, time::Instant};

use crate::{
    command_handler::CommandHandler,
    http::HTTPRequest,
    errors::{
        CommandError,
        ParseError,
    },
};

#[derive(Debug, Default)]
pub struct Installer {
    package_author: String,
    package_name: String,
}

impl Installer {
    fn parse_package_details(package_details: String) -> Result<(String, String), ParseError> {
        let mut split = package_details.split('/');
        split.next();

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

#[async_trait]
impl CommandHandler for Installer {
    fn parse(&mut self, args: &mut Args) -> Result<(), ParseError> {
        let package_details = args
            .next()
            .ok_or(ParseError::MissingArgument(String::from("package name")))?;

        let (package_author, package_name) = Self::parse_package_details(package_details)?;
        self.package_name = package_name;
        self.package_author = package_author;

        Ok(())
    }

    async fn execute(&self) -> Result<(), CommandError> {
        println!("Installing '{}'..", self.package_name);

        let client = reqwest::Client::new();
        let now = Instant::now();

        let package_data =
            HTTPRequest::get_verilog_files(client.clone(), &self.package_author, &self.package_name).await?;
        Self::install_package(client).await?;

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());

        Ok(())
    }
}