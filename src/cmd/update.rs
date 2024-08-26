use std::path::PathBuf;
use std::fs;
use anyhow::Result;

use crate::cmd::{Execute, Update};
use crate::cmd::include::include_module_from_url;

impl Execute for Update {
    fn execute(&self) -> Result<()> {
        let package_name = &self.package_path;
        println!("Updating package '{}'", package_name);
        update_package(package_name)
    }
}

fn update_package(package_name: &str) -> Result<()> {
    Ok(())
}