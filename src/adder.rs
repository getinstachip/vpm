use async_trait::async_trait;
use std::{fs, path::Path, time::Instant};
use toml_edit::{Document, Item, value};

use crate::{
    errors::CommandError,
    CommandHandler,
};

#[derive(Debug, Default)]
pub struct Adder {
    package_path: String,
    collection_name: String,
}

impl Adder {
    pub fn new(package_path: String, collection_name: String) -> Self {
        Self {
            package_path,
            collection_name,
        }
    }

    fn add_package_to_collection(&self) -> Result<(), CommandError> {
        let package_name = Path::new(&self.package_path)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| CommandError::InvalidPath("Invalid package path".to_string()))?;

        let collections_toml_path = format!("./{}.Collections.toml", self.collection_name);
        let mut doc = fs::read_to_string(&collections_toml_path)
            .map_err(|e| CommandError::IOError(e))?
            .parse::<Document>()
            .map_err(|e| CommandError::ParseError(e.to_string()))?;

        if let Some(packages) = doc.as_table_mut().get_mut("packages") {
            if let Some(packages_table) = packages.as_table_mut() {
                packages_table.insert(package_name, value(self.package_path.clone()));
            } else {
                return Err(CommandError::ParseError("[packages] is not a table".to_string()));
            }
        } else {
            doc["packages"] = Item::Table(toml_edit::Table::new());
            doc["packages"][package_name] = value(self.package_path.clone());
        }

        fs::write(collections_toml_path, doc.to_string())
            .map_err(|e| CommandError::IOError(e))?;

        Ok(())
    }
}

#[async_trait]
impl CommandHandler for Adder {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();

        self.add_package_to_collection()?;

        println!("Package '{}' added to collection '{}'", self.package_path, self.collection_name);
        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }
}
