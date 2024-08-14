use anyhow::Result;

use crate::cmd::{Execute, Init};
use crate::versions::versions::{create_toml, update_library_entry, update_config_entry, write_file};

impl Execute for Init {
    fn execute(&self) -> Result<()> {
        let mut toml_doc = create_toml(false)?;
        let mut lock_doc = create_toml(true)?;
        
        // .toml
        update_library_entry(&mut toml_doc,
                             Some(&self.project_name),
                             self.version.as_deref(),
                             self.description.as_deref(),
                             self.authors.as_deref(),
                             self.license.as_deref(),
                             None)?;
        // .lock
        update_library_entry(&mut lock_doc,
                             Some(&self.project_name),
                             self.version.as_deref(),
                             self.description.as_deref(),
                             self.authors.as_deref(),
                             self.license.as_deref(),
                             None)?;

        // docs
        update_config_entry(&mut toml_doc, "docs", "override_docs_path", toml_edit::Value::from(""))?;
        update_config_entry(&mut lock_doc, "docs", "override_docs_path", toml_edit::Value::from(""))?;

        write_file(toml_doc, false)?;
        write_file(lock_doc, true)?;

        Ok(())
    }
    
}