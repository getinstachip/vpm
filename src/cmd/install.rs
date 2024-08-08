use anyhow::Result;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        if let Some(url) = &self.url {
            // TODO: Add url install logic
        } else if let Some(name) = &self.package_name {
            // TODO: Add package install logic
        }

        Ok(())
    }
}
