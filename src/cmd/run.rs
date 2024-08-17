use anyhow::Result;

use crate::cmd::{Execute, Run};

impl Execute for Run {
    fn execute(&self) -> Result<()> {
        println!("Running project...");
        Ok(())
    }
}
