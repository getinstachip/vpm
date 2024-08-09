use anyhow::{Context, Result};
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use anyhow::bail;
use chatgpt::prelude::*;

use crate::cmd::{Execute, Install};

const LOCATION: &str = "./vpm_modules";

impl Execute for Install {
    async fn execute(&self) -> Result<()> {
        if let Some(url) = &self.url {
            install_from_url(url, LOCATION)?;
        } else if let Some(name) = &self.package_name {
            download_documentation(name).await?;
        }

        Ok(())
    }
}

fn install_from_url(url: &String, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default()
    );

    dbg!(url.split('/').last().unwrap_or_default());
    Command::new("git")
        .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path.to_str().unwrap_or_default()])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}

async fn download_documentation(package_name: &str) -> Result<()> {
    let package_path = Path::new(LOCATION).join(package_name);
    if !package_path.exists() {
        bail!("Package '{}' not found in {}", package_name, LOCATION);
    }

    let readme_path = package_path.join("README.md");
    let content = if readme_path.exists() {
        fs::read_to_string(readme_path)?
    } else {
        // If README.md doesn't exist, try to find a Verilog file
        let verilog_files: Vec<_> = fs::read_dir(&package_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "v" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if verilog_files.is_empty() {
            bail!("No README.md or Verilog files found in package '{}'", package_name);
        }

        fs::read_to_string(&verilog_files[0])?
    };
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let openai_client = ChatGPT::new(key)?;
    let response = openai_client.send_message(content).await?;
    let content = response.message().content.clone();

    // Save the generated documentation to a README.md file
    let readme_path = package_path.join("README.md");
    fs::write(&readme_path, content)?;

    println!("Documentation saved to: {}", readme_path.display());

    Ok(())
}