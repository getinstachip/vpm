use anyhow::Result;

use crate::cmd::{Install, Execute};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        // TODO: Add logic for install
        Ok(())
    }
}
