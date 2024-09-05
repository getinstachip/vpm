use anyhow::Result;

use crate::cmd::{Execute, Run};

impl Execute for Run {
    async fn execute(&self) -> Result<()> {
        Ok(())
    }
}