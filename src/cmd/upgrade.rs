use anyhow::Result;
use std::process::Command;

use crate::cmd::{Execute, Upgrade};

impl Execute for Upgrade {
    async fn execute(&self) -> Result<()> {
        println!("Upgrading VPM...");
        
        upgrade_vpm()?;
        
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