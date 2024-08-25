use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;

use crate::cmd::{Execute, Run};

impl Execute for Run {
    fn execute(&self) -> Result<()> {
        println!("Running simulation for core: {}", self.core_name);

        let mut args = vec![
            "run".to_string(),
            format!("--target={}", self.target),
            format!("--tool={}", self.tool),
            self.core_name.clone(),
        ];

        args.extend(self.additional_args.clone());

        let output = Command::new("fusesoc")
            .args(&args)
            .output()?;

        if !output.status.success() {
            println!("FuseSoC error: {}", String::from_utf8_lossy(&output.stderr));
            return Err(anyhow::anyhow!("FuseSoC command failed"));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}