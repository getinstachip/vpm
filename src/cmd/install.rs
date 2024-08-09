use anyhow::{Context, Result};
use std::{fs, process::Command};
use std::path::PathBuf;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        const LOCATION: &str = "./vpm_modules";

        if let (Some(url), Some(module)) = (&self.url, &self.package_name) {
            install_module_from_url(url, module)?;
        } else if let Some(url) = &self.url {
            install_from_url(url, LOCATION)?;
        } else if let Some(_name) = &self.package_name {
            // TODO: Add package install logic
        }

        Ok(())
    }
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

fn install_module_from_url(url: &String, module: &String) -> Result<()> {
    let temp_dir = format!("/tmp/{}", module);
    install_from_url(url, &temp_dir)?;
    search_for_module(&temp_dir, module)?;
    Ok(())
}

fn search_for_module(dir: &str, module: &str) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == module) {
            // TODO: Search each dependency in module
        } else if path.is_dir() {
            search_for_module(path.to_str().unwrap_or_default(), module)?;
        }
    }
    Ok(())
}

fn install_from_url(url: &String, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        name_from_url(url).with_context(|| format!("Failed to parse URL: '{}'", url))?
    );

    Command::new("git")
        .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path.to_str().unwrap_or_default()])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}
