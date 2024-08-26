use anyhow::Result;

use crate::cmd::{Execute, Update};

impl Execute for Update {
    fn execute(&self) -> Result<()> {
        let package_name = &self.package_path;
        println!("Updating package '{}'", package_name);
        update_package(package_name)
    }
}

fn update_package(_package_name: &str) -> Result<()> {
    Ok(())
}