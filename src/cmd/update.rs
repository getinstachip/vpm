use std::path::PathBuf;
use std::fs;
use anyhow::Result;

use crate::cmd::{Execute, Update};
use crate::toml::{get_dependency, update_dependency};
use crate::cmd::include::include_module_from_url;

impl Execute for Update {
    fn execute(&self) -> Result<()> {
        let package_name = &self.package_name;
        println!("Updating package '{}'", package_name);
        update_package(package_name)
    }
}

fn update_package(package_name: &str) -> Result<()> {
    let package_path = PathBuf::from("./vpm_modules").join(package_name);
    if !package_path.exists() {
        return Err(anyhow::anyhow!("Package '{}' not found", package_name));
    }

    let dependency = get_dependency(package_name).expect("Dependency should exist for an installed package");
    let url = dependency.get_url().ok_or_else(|| anyhow::anyhow!("URL not found for package '{}'", package_name))?;

    fs::remove_dir_all(&package_path)?;
    include_module_from_url(package_name, &url)?;
    update_dependency(package_name, Some(&url), None, Some(package_name))?;

    println!("Package '{}' updated successfully", package_name);
    Ok(())
}