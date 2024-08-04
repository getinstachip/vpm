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
        // split.next();

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

    async fn install_package(
        client: reqwest::Client,
        package_author: &str,
        package_name: &str,
    ) -> Result<(), CommandError> {
        let verilog_files = HTTPRequest::get_verilog_files(client.clone(), package_author, package_name).await?;

        use std::fs;
        use std::path::Path;

        let vpm_modules_dir = Path::new("./vpm_modules");
        if !vpm_modules_dir.exists() {
            fs::create_dir_all(vpm_modules_dir).map_err(CommandError::IOError)?;
        }

        for file in verilog_files {
            if let Some(download_url) = file.download_url {
                let content = client
                    .get(&download_url)
                    .send()
                    .await
                    .map_err(CommandError::HTTPFailed)?
                    .text()
                    .await
                    .map_err(CommandError::FailedResponseText)?;

                let file_path = vpm_modules_dir.join(&file.name);
                fs::write(&file_path, content).map_err(|e| CommandError::IOError(e))?;

                println!("Downloaded file: {}", file.name);
            }
        }

        Ok(())
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

        Self::install_package(client.clone(), &self.package_author, &self.package_name).await?;

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());

        Ok(())
    }
}