use anyhow::Result;

use crate::cmd::{Execute, Include};

impl Execute for Include {
    fn execute(&self) -> Result<()> {
        // TODO: Add Include logic
        Ok(())
    }
}
