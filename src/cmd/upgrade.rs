use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
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
    let output = Command::new("sh")
        .arg("-c")
        .arg("curl -sSfL https://raw.githubusercontent.com/getinstachip/vpm-internal/main/install.sh | sh")
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Upgrade command failed"));
    }

    Ok(())
}