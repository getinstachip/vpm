use async_trait::async_trait;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env::Args, fs, path::Path, time::Instant};
use uuid::Uuid;

use crate::http::GitHubFile;
use crate::{
    command_handler::CommandHandler,
    errors::{CommandError, ParseError},
    http::HTTPRequest,
    embedding::{create_client, create_index, embed_library, insert_documents},
};


#[derive(Debug, Default)]
pub struct Installer {
    package_author: String,
    package_name: String,
    flex_install: bool,
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
        package_name: String,
        verilog_files: Vec<GitHubFile>,
        context: bool,
    ) -> Result<(), CommandError> {
        let vpm_modules_dir = Path::new("./vpm_modules");
        if !vpm_modules_dir.exists() {
            fs::create_dir_all(vpm_modules_dir).map_err(CommandError::IOError)?;
        }
        fs::create_dir_all(vpm_modules_dir.join(&package_name)).map_err(CommandError::IOError)?;

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

                let file_path = vpm_modules_dir.join(format!("{}/{}", package_name, &file.name));
                if context {
                    // Tune context to codebase
                }
                fs::write(&file_path, content).map_err(CommandError::IOError)?;
                pb.set_message(format!("Downloading: {}", file.name));
                pb.inc(1);
            }
        }

        pb.finish_with_message("✨ All files downloaded successfully!");
        Ok(())
    }

    async fn embed_codebase() -> Result<(), CommandError> {
        println!("Performing flex install: Embedding and storing codebase...");
        let es_client = match create_client() {
            Ok(client) => client,
            Err(e) => return Err(CommandError::ElasticsearchConnectionError(e.to_string())),
        };
        let random_key = Uuid::new_v4().to_string();
        let stripped_key = random_key.replace(&['-', '_'][..], "");
        let index_name = format!("codebase{}", stripped_key).to_lowercase();
        println!("Creating index: {}", index_name);            
        match create_index(&es_client, &index_name).await {
            Ok(_) => println!("Index '{}' created successfully", index_name),
            Err(e) => return Err(CommandError::ElasticsearchConnectionError(format!("Failed to create index: {}", e))),
        }
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);
        let documents = embed_library(&current_dir, &index_name).await.unwrap();
        println!("Number of embedded documents: {}", documents.len());
        insert_documents(&es_client, &index_name, &documents).await.unwrap();
        println!("Codebase embedded and stored successfully!");
        Ok(())
    }
}

#[async_trait]
impl CommandHandler for Installer {
    fn parse(&mut self, args: &mut Vec<String>) -> Result<(), ParseError> {
        let parts: Vec<&str> = args[0].split("/").collect();
        let (package_author, package_name) = (parts[0], parts[1]);
        self.package_name = package_name.to_string();
        self.package_author = package_author.to_string();

        Ok(())
    }

    fn parse_flags(&mut self, flags: &mut Vec<String>) -> Result<(), ParseError> {
        for flag in flags {
            if flag == "--flex" {
                self.flex_install = true;
            }
        }
        Ok(())
    }

    async fn execute(&self) -> Result<(), CommandError> {
        let client = reqwest::Client::new();
        let now = Instant::now();

        let verilog_files = HTTPRequest::get_verilog_files(
            client.clone(),
            self.package_author.to_string(),
            self.package_name.to_string(),
        )
        .await?;
        let vpm_toml_path = std::path::Path::new("./Vpm.toml");
        if !vpm_toml_path.exists() {
            std::fs::File::create(vpm_toml_path).unwrap();
            println!("Created Vpm.toml file");
        }
        let mut vpm_toml_content = std::fs::read_to_string(vpm_toml_path).unwrap();
        if !vpm_toml_content.contains("[dependencies]") {
            vpm_toml_content.push_str("[dependencies]\n");
        }
        if self.flex_install {
            Self::embed_codebase().await?;
        }
        Self::install_package(client.clone(), self.package_name.to_string(), verilog_files, self.flex_install).await?;
        vpm_toml_content.push_str(&format!("{}/{}\n", self.package_author, self.package_name));
        std::fs::write(vpm_toml_path, vpm_toml_content).unwrap();
        println!("Package '{}' added to Vpm.toml", self.package_name);
        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }
}
