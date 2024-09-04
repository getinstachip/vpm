use anyhow::Result;

use crate::cmd::{Execute, Update};

impl Execute for Update {
    async fn execute(&self) -> Result<()> {
        let module_path = &self.module_path;
        println!("Updating module '{}'", module_path);
        update_module(module_path)
    }
}

fn update_module(_module_path: &str) -> Result<()> {
    Ok(())
}