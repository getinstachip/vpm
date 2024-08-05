use async_trait::async_trait;
use std::{fs, path::Path, time::Instant};
use std::io::{BufRead, BufReader, Write};

use crate::errors::{CommandError, ParseError};
use crate::command_handler::CommandHandler;

#[derive(Debug, Default)]
pub struct Remover {
    package_author: String,
    package_name: String,
}

impl Remover {
    pub fn new(repo: String) -> Self {
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
        }
    }

    fn remove_package(package_name: &str) -> Result<(), CommandError> {
        let vpm_modules_dir = Path::new("./vpm_modules");
        let package_dir = vpm_modules_dir.join(package_name);

        if package_dir.exists() {
            fs::remove_dir_all(&package_dir).map_err(CommandError::IOError)?;
            println!("Removed package directory: {:?}", package_dir);
        } else {
            println!("Package directory does not exist: {:?}", package_dir);
        }

        Ok(())
    }

    fn remove_from_vpm_toml(&self) -> Result<bool, CommandError> {
        let vpm_toml_path = Path::new("./Vpm.toml");
        if !vpm_toml_path.exists() {
            return Ok(false);
        }

        let file = fs::File::open(vpm_toml_path).map_err(CommandError::IOError)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        let package_line = format!("{}/{} = ", self.package_author, self.package_name);
        let package_exists = lines.iter().any(|line| line.starts_with(&package_line));

        if package_exists {
            let updated_lines: Vec<String> = lines
                .into_iter()
                .filter(|line| !line.starts_with(&package_line))
                .collect();

            let mut file = fs::File::create(vpm_toml_path).map_err(CommandError::IOError)?;
            for line in updated_lines {
                writeln!(file, "{}", line).map_err(CommandError::IOError)?;
            }

            println!("Removed package from vpm.toml");
            Ok(true)
        } else {
            println!("Package {}/{} not found in vpm.toml", self.package_author, self.package_name);
            Ok(false)
        }
    }
}

#[async_trait]
impl CommandHandler for Remover {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();

        Self::remove_package(&self.package_name)?;

        let elapsed = now.elapsed();
        if self.remove_from_vpm_toml()? {
            println!("Package '{}' removed successfully", self.package_name);
        } else {
            println!("Package '{}' not found", self.package_name);
        }
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
