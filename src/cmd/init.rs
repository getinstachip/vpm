use anyhow::Result;
use std::fs::File;
use std::io::Write;

use crate::cmd::{Execute, Init};
#[path = "../versions.rs"]
mod versions;

impl Execute for Init {
    fn execute(&self) -> Result<()> {
        let mut toml_doc = versions::create_toml(false)?;
        let mut lock_doc = versions::create_toml(true)?;
        
        // .toml
        versions::update_library_entry(&mut toml_doc,
                                       Some(&self.project_name),
                                       self.version.as_deref(),
                                       self.description.as_deref(),
                                       self.authors.as_deref(),
                                       self.license.as_deref(),
                                       None)?;
        // .lock
        versions::update_library_entry(&mut lock_doc,
                                       Some(&self.project_name),
                                       self.version.as_deref(),
                                       self.description.as_deref(),
                                       self.authors.as_deref(),
                                       self.license.as_deref(),
                                       None)?;

        let toml_str = toml_doc.to_string();
        let mut toml_file = File::create(versions::VPM_TOML).expect("Failed to create vpm.toml");
        toml_file.write_all(toml_str.as_bytes()).expect("Failed to write to vpm.toml");

        let lock_str = lock_doc.to_string();
        let mut lock_file = File::create(versions::VPM_LOCK).expect("Failed to create vpm.lock");
        lock_file.write_all(lock_str.as_bytes()).expect("Failed to write to vpm.lock");

        Ok(())
    }
    
}