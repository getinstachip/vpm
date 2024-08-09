use anyhow::{Context, Result};
use std::process::Command;
use std::path::PathBuf;

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
