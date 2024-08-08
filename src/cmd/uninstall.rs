use anyhow::Result;

use crate::cmd::{Execute, Uninstall};

impl Execute for Uninstall {
    fn execute(&self) -> Result<()> {
        // TODO: Write uninstall logic
        Ok(())
    }
}
