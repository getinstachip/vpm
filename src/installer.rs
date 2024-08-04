use async_trait::async_trait;
use std::{env::Args, time::Instant};
use indicatif::{ProgressBar, ProgressStyle};

use crate::http::GitHubFile;
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
        verilog_files: Vec<GitHubFile>,
    ) -> Result<(), CommandError> {
        use std::fs;
        use std::path::Path;

        let vpm_modules_dir = Path::new("./vpm_modules");
        if !vpm_modules_dir.exists() {
            fs::create_dir_all(vpm_modules_dir).map_err(CommandError::IOError)?;
        }

        let pb = ProgressBar::new(verilog_files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("=> "));

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

                pb.set_message(format!("Downloading: {}", file.name));
                pb.inc(1);
            }
        }

        pb.finish_with_message("✨ All files downloaded successfully!");
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
        let client = reqwest::Client::new();
        let now = Instant::now();

        let verilog_files = HTTPRequest::get_verilog_files(client.clone(), self.package_author.to_string(), self.package_name.to_string()).await?;
        Self::install_package(client.clone(), verilog_files).await?;

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());

        Ok(())
    }
}