use async_trait::async_trait;
use std::{fs, path::Path, time::Instant, io::Read};
use std::io::{BufRead, BufReader, Write};
use std::io::Seek;

use crate::errors::CommandError;
use crate::CommandHandler;
use crate::http::HTTPRequest;
use crate::installer::Installer;

#[derive(Debug, Default)]
pub struct Updater {
    package_author: String,
    package_name: String,
    flex_update: bool,
}

impl Updater {
    pub fn new(repo: Option<String>, flex_update: bool) -> Self {
        if let Some(repo) = repo {
            let mut split = repo.split('/');

            let package_author = split
                .next()
                .expect("Provided package author is empty")
                .to_string();

            let package_name = split
                .next()
                .expect("Provided package name is empty")
                .to_string();

            Self {
                package_author,
                package_name,
                flex_update,
            }
        } else {
            Self {
                package_author: String::new(),
                package_name: String::new(),
                flex_update,
            }
        }
    }

    async fn check_update_available(&self, current_commit: &str) -> Result<bool, CommandError> {
        let client = reqwest::Client::new();
        let latest_commit = HTTPRequest::get_latest_commit_id(
            client,
            self.package_author.clone(),
            self.package_name.clone(),
        )
        .await?;

        if latest_commit != current_commit {
            println!("Update available for {}/{}:", self.package_author, self.package_name);
            println!("Current commit: {}", current_commit);
            println!("Latest commit:  {}", latest_commit);
            Ok(true)
        } else {
            println!("Package {}/{} is up to date", self.package_author, self.package_name);
            Ok(false)
        }
    }

    async fn update_package(&self) -> Result<(), CommandError> {
        let client = reqwest::Client::new();
        let latest_commit = HTTPRequest::get_latest_commit_id(
            client.clone(),
            self.package_author.clone(),
            self.package_name.clone(),
        )
        .await?;

        let vpm_toml_path = Path::new("./Vpm.toml");
        if !vpm_toml_path.exists() {
            return Err(CommandError::MissingFile("Vpm.toml not found".to_string()));
        }

        let content = fs::read_to_string(vpm_toml_path).map_err(CommandError::IOError)?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        let package_line = format!("{}/{} = ", self.package_author, self.package_name);
        let package_index = lines.iter().position(|line| line.starts_with(&package_line));

        if let Some(index) = package_index {
            let mut file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(vpm_toml_path)
                .map_err(CommandError::IOError)?;

            let mut content = String::new();
            file.read_to_string(&mut content).map_err(CommandError::IOError)?;

            let package_line = format!("{}/{} = ", self.package_author, self.package_name);
            let updated_content = content.lines().map(|line| {
                if line.starts_with(&package_line) {
                    format!("{}/{} = \"{}\"", self.package_author, self.package_name, latest_commit)
                } else {
                    line.to_string()
                }
            }).collect::<Vec<String>>().join("\n");

            file.set_len(0).map_err(CommandError::IOError)?;
            file.seek(std::io::SeekFrom::Start(0)).map_err(CommandError::IOError)?;
            file.write_all(updated_content.as_bytes()).map_err(CommandError::IOError)?;

            // Remove old package files
            let vpm_modules_dir = Path::new("./vpm_modules");
            let package_dir = vpm_modules_dir.join(&self.package_name);
            if package_dir.exists() {
                fs::remove_dir_all(&package_dir).map_err(CommandError::IOError)?;
            }

            // Install new package files
            let installer = Installer::new(format!("{}/{}", self.package_author, self.package_name), self.flex_update);
            installer.execute().await?;

            println!("Package '{}/{}' updated successfully", self.package_author, self.package_name);
            Ok(())
        } else {
            println!("Package '{}/{}' not found in Vpm.toml", self.package_author, self.package_name);
            Ok(())
        }
    }

    async fn update_all_packages(&self) -> Result<(), CommandError> {
        let vpm_toml_path = Path::new("./Vpm.toml");
        if !vpm_toml_path.exists() {
            return Err(CommandError::MissingFile("Vpm.toml not found".to_string()));
        }

        let content = fs::read_to_string(vpm_toml_path).map_err(CommandError::IOError)?;
        let lines: Vec<String> = content.lines().map(String::from).collect();

        for line in lines {
            if line.contains(" = ") {
                let parts: Vec<&str> = line.split(" = ").collect();
                if parts.len() == 2 {
                    let repo = parts[0].trim();
                    let updater = Updater::new(Some(repo.to_string()), self.flex_update);
                    updater.update_package().await?;
                }
            }
        }

        println!("All packages updated successfully");
        Ok(())
    }
}

#[async_trait]
impl CommandHandler for Updater {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();

        if self.package_name.is_empty() {
            self.update_all_packages().await?;
        } else {
            let vpm_toml_path = Path::new("./Vpm.toml");
            if !vpm_toml_path.exists() {
                return Err(CommandError::MissingFile("Vpm.toml not found".to_string()));
            }

            let content = fs::read_to_string(vpm_toml_path).map_err(CommandError::IOError)?;
            let lines: Vec<String> = content.lines().map(String::from).collect();

            let package_line = format!("{}/{} = ", self.package_author, self.package_name);
            if let Some(line) = lines.iter().find(|line| line.starts_with(&package_line)) {
                let current_commit = line.split('"').nth(1).unwrap_or("");
                if let Ok(available) = self.check_update_available(current_commit).await {
                    if available {
                        self.update_package().await?;
                    } else {
                        println!("Package '{}/{}' is already up to date", self.package_author, self.package_name);
                    }
                } else {
                    println!("Failed to check for updates for package '{}/{}'", self.package_author, self.package_name);
                }
            } else {
                println!("Package '{}/{}' not found in Vpm.toml", self.package_author, self.package_name);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }
}