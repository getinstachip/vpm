use anyhow::Result;

use crate::cmd::{Execute, Init};
use crate::versions::versions::{create_toml, update_library_entry, update_config_entry};

impl Execute for Init {
    fn execute(&self) -> Result<()> {
        create_toml(false)?;
        create_toml(true)?;
        
        // .toml
        update_library_entry(false,
                             Some(&self.project_name),
                             self.version.as_deref(),
                             self.description.as_deref(),
                             self.authors.as_deref(),
                             self.license.as_deref(),
                             None)?;
        // .lock
        update_library_entry(true,
                             Some(&self.project_name),
                             self.version.as_deref(),
                             self.description.as_deref(),
                             self.authors.as_deref(),
                             self.license.as_deref(),
                             None)?;

        // docs
        update_config_entry(false, "docs", "override_docs_path", toml_edit::Value::from(""))?;
        update_config_entry(true, "docs", "override_docs_path", toml_edit::Value::from(""))?;

        Ok(())
    }
    
}