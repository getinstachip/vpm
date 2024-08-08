use anyhow::Result;

use crate::cmd::{Install, Run};

impl Run for Install {
    fn run(&self) -> Result<()> {
        // TODO: Add logic for install
        Ok(())
    }
}
