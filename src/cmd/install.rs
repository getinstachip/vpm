use anyhow::{Context, Result};
use std::process::Command;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        const LOCATION: &str = "./vpm_modules";

        if let Some(url) = &self.url {
            install_from_url(url, LOCATION)?;
        } else if let Some(_name) = &self.package_name {
            // TODO: Add package install logic
        }

        Ok(())
    }
}

fn install_from_url(url: &String, location: &str) -> Result<()>{
    Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--single-branch")
        .arg("--jobs")
        .arg("4")
        .arg(url)
        .arg(location)
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;
    
    Ok(())
}
