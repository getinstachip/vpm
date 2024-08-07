use async_trait::async_trait;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, time::Instant};
use crate::errors::ParseError;

use crate::http::GitHubFile;
use crate::{
    errors::CommandError,
    http::HTTPRequest,
    command_handler::CommandHandler,
};
use crate::headers::generate_header;

#[derive(Debug, Default)]
pub struct Installer {
    package_author: String,
    package_name: String,
    flex_install: bool,
    version: String,
}

impl Installer {
    pub fn new(repo: String, flex_install: bool) -> Self {
        let mut split = repo.split('/');

        let package_author = split
            .next()
            .expect("Provided package author is empty")
            .to_string();

        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        
        let mut split = name.split('@');
        let package_name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        let version = split
            .next()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "latest".to_string());

        Self {
            package_author,
            package_name,
            flex_install,
            version,
        }
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
        let package_dir = vpm_modules_dir.join(&package_name);
        let files_dir = package_dir.join("files");
        fs::create_dir_all(&files_dir).map_err(CommandError::IOError)?;

        let pb = ProgressBar::new(verilog_files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .progress_chars("=> ")
        );

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

                // Preserve directory structure
                let relative_path = Path::new(&file.path);
                let file_path = files_dir.join(relative_path);
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).map_err(CommandError::IOError)?;
                }

                if context {
                    // Tune context to codebase
                }
                
                // Create a new folder for the file pair
                if file.name.ends_with(".v") {
                    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
                    let pair_folder = file_path.parent().unwrap().join(file_name);
                    fs::create_dir_all(&pair_folder).map_err(CommandError::IOError)?;

                    // Write the .v file
                    let v_file_path = pair_folder.join(&file.name);
                    fs::write(&v_file_path, &content).map_err(CommandError::IOError)?;
                    
                    // Generate and write header file
                    let header_content = generate_header(&content, &file.name);
                    let header_name = file.name.replace(".v", ".vh");
                    let header_path = pair_folder.join(header_name);
                    fs::write(&header_path, header_content).map_err(CommandError::IOError)?;
                } else {
                    // For non-.v files, just write the file directly
                    fs::write(&file_path, &content).map_err(CommandError::IOError)?;
                }

                pb.set_message(format!("Downloading: {}", file.path));
                pb.inc(1);
            }
        }

        pb.finish_with_message("✨ All files downloaded successfully!");
        Ok(())
    }

}

#[async_trait]
impl CommandHandler for Installer {
    async fn execute(&self) -> Result<(), CommandError> {
        let client = reqwest::Client::new();
        let now = Instant::now();
        let version = if self.version == "latest" {
            HTTPRequest::get_latest_commit_id(
                client.clone(),
                self.package_author.to_string(),
                self.package_name.to_string(),
            )
            .await?
        } else {
            self.version.clone()
        };

        let verilog_files = HTTPRequest::get_verilog_files(
            client.clone(),
            self.package_author.to_string(),
            self.package_name.to_string(),
        )
        .await?;
        let vpm_toml_path = std::path::Path::new("./vpm.toml");
        if !vpm_toml_path.exists() {
            std::fs::File::create(vpm_toml_path).unwrap();
            println!("Created vpm.toml file");
        }
        let mut vpm_toml_content = std::fs::read_to_string(vpm_toml_path).unwrap();
        if !vpm_toml_content.contains("[dependencies]") {
            vpm_toml_content.push_str("[dependencies]\n");
        }
        Self::install_package(
            client.clone(),
            self.package_name.to_string(),
            verilog_files,
            self.flex_install,
        )
        .await?;
        vpm_toml_content.push_str(&format!("{}/{} = \"{}\"\n", self.package_author, self.package_name, version));
        std::fs::write(vpm_toml_path, vpm_toml_content).unwrap();
        println!("Package '{}' added to vpm.toml", self.package_name);
        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }

    async fn list() -> Result<(), ParseError> {
        let vpm_toml_path = Path::new("./vpm.toml");
        if !vpm_toml_path.exists() {
            println!("No packages installed. vpm.toml file not found.");
            return Ok(());
        }

        let vpm_toml_content = fs::read_to_string(vpm_toml_path)
            .map_err(|e| ParseError::MissingArgument(format!("Failed to read vpm.toml: {}", e)))?;

        let mut found_dependencies = false;
        for line in vpm_toml_content.lines() {
            if line.trim() == "[dependencies]" {
                found_dependencies = true;
                println!("Installed packages:");
                continue;
            }
            if found_dependencies && !line.trim().is_empty() {
                println!("  {}", line.trim());
            }
        }

        if !found_dependencies {
            println!("No packages installed.");
        }

        Ok(())
    }
}
