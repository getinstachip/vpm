use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::cmd::{Execute, Uninstall};

impl Execute for Uninstall {
    fn execute(&self) -> Result<()> {
        fs::remove_dir_all(PathBuf::from("./vpm_modules").join(&self.package_path))?;
        Ok(())
    }
}
