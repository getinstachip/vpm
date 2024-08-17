use anyhow::Result;

use crate::resolver::resolve_dependencies;
use crate::cmd::{Execute, Run};

impl Execute for Run {
    fn execute(&self) -> Result<()> {
        println!("Generating lock file...");
        resolve_dependencies()
    }
}
