use anyhow::Result;
use std::process::Command;

use crate::cmd::{Execute, Upgrade};
use crate::config_man::set_version;

impl Execute for Upgrade {
    async fn execute(&self) -> Result<()> {
        println!("Upgrading VPM...");
        
        upgrade_vpm()?;
        set_version(&get_latest_version()?)?;

        println!("VPM upgrade completed successfully.");
        Ok(())
    }
}

fn upgrade_vpm() -> Result<()> {
    if cfg!(unix) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -sSfL https://raw.githubusercontent.com/getinstachip/vpm-internal/main/install.sh | sh")
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Upgrade command failed"));
        }
    } else if cfg!(windows) {
        println!("To upgrade VPM on Windows, please follow these steps:");
        println!("1. Visit https://github.com/getinstachip/vpm/releases/latest");
        println!("2. Download the appropriate .exe file for your system");
        println!("3. Run the downloaded .exe file to complete the upgrade");
        return Ok(());
    } else {
        return Err(anyhow::anyhow!("Unsupported operating system"));
    }

    Ok(())
}

fn get_latest_version() -> Result<String> {
    let output = Command::new("git")
        .arg("describe ")
        .arg("--tags")
        .arg("--abbrev=0")
        .output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get latest version"));
    }
    Ok(String::from_utf8(output.stdout)?)
}