use async_trait::async_trait;
use std::{fs, path::Path, time::Instant};
use uuid::Uuid;

use crate::{
    errors::CommandError,
    CommandHandler,
};

#[derive(Debug, Default)]
pub struct CollectionCreator {
    id: String,
    name: String,
}

impl CollectionCreator {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}

#[async_trait]
impl CommandHandler for CollectionCreator {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();
        let collections_toml_path = format!("./{}.Collections.toml", self.name);
        let path = Path::new(&collections_toml_path);
        let vpm_toml_path = Path::new("Vpm.toml");

        // Check if collection already exists in Vpm.toml
        let vpm_toml_content = fs::read_to_string(vpm_toml_path)
            .map_err(|e| CommandError::IOError(e))?;
        let collection_exists_in_vpm = vpm_toml_content.contains(&format!("{} = ", self.name));

        if path.exists() && collection_exists_in_vpm {
            println!("Collection '{}' already exists.", self.name);
            return Ok(());
        }

        // Create new collection if it doesn't exist
        if !path.exists() {
            std::fs::File::create(path).map_err(|e| CommandError::IOError(e))?;
            println!("Created {}.Collections.toml file", self.name);
        }

        let mut collections_toml_content = std::fs::read_to_string(path)
            .map_err(|e| CommandError::IOError(e))?;

        if !collections_toml_content.contains("[details]") {
            collections_toml_content.push_str("[details]\n");
            collections_toml_content.push_str(&format!("name = \"{}\"\n", self.name));
            collections_toml_content.push_str(&format!("id = \"{}\"\n", self.id));
            collections_toml_content.push_str("\n");
        }

        // Add entry to Vpm.toml under [collections] field if it doesn't exist
        if !collection_exists_in_vpm {
            let mut vpm_toml_content = vpm_toml_content;
            let collection_entry = format!("{} = \"{}\"\n", self.name, self.id);

            if vpm_toml_content.contains("[collections]") {
                // If [collections] exists, find its position
                if let Some(collections_index) = vpm_toml_content.find("[collections]") {
                    // Insert the new entry right after [collections]
                    vpm_toml_content.insert_str(collections_index + "[collections]\n".len(), &collection_entry);
                }
            } else {
                // If [collections] doesn't exist, add it at the beginning of the file
                vpm_toml_content = format!("[collections]\n{}{}", collection_entry, vpm_toml_content);
            }

            fs::write(vpm_toml_path, vpm_toml_content)
                .map_err(|e| CommandError::IOError(e))?;

            println!("Added collection '{}' to Vpm.toml", self.name);
        }

        if !collections_toml_content.contains("[packages]") {
            collections_toml_content.push_str("[packages]\n");
        }
        std::fs::write(path, collections_toml_content).map_err(|e| CommandError::IOError(e))?;
        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }
}