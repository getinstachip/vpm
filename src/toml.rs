use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use anyhow::Result;
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};


#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license: String,
}

#[derive(Debug)]
struct VpmToml {
    toml_doc: DocumentMut,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: "my-vpm-package".to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["<author-name> <author-email>".to_string()],
            description: "A vpm package".to_string(),
            license: "LicenseRef-LICENSE".to_string(),
        }
    }
}

impl VpmToml {    
    pub fn from(filepath: &str) -> Self {
        if !Path::new(filepath).exists() {
            let mut initial_doc = DocumentMut::new();
            initial_doc["package"] = Item::Table(Table::new());
            initial_doc["package"]["name"] = Item::Value(Value::from(Package::default().name));
            initial_doc["package"]["version"] = Item::Value(Value::from(Package::default().version));
            initial_doc["package"]["authors"] = Item::Value(Value::from(Array::from(Package::default().authors.iter().map(|s| Value::from(s.to_string())).collect())));
            initial_doc["package"]["description"] = Item::Value(Value::from(Package::default().description));
            initial_doc["package"]["license"] = Item::Value(Value::from(Package::default().license));

            initial_doc["dependencies"] = Item::Table(Table::new());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filepath)
                .expect("Failed to create vpm.toml");
            file.write_all(initial_doc.to_string().as_bytes()).expect("Failed to write to vpm.toml");
        }

        let toml_content = read_to_string(filepath).expect("Failed to read vpm.toml");
        Self {
            toml_doc: toml_content.parse::<DocumentMut>().expect("Failed to parse vpm.toml")
        }
    }

    pub fn get_dependencies(&self) -> Option<&Table> {
        self.toml_doc["dependencies"].as_table()
    }

    pub fn add_dependency(&mut self, git: &str, commit: Option<&str>) {
        let mut dependency = InlineTable::new();
        dependency.insert("top_modules", Value::Array(Array::new()));
        dependency.insert("commit", Value::from(commit.unwrap_or_default().to_string()));
        self.toml_doc["dependencies"][git] = Item::Value(Value::InlineTable(dependency));
    }

    pub fn add_top_module(&mut self, repo_link: &str, module_name: &str) {
        self.toml_doc["dependencies"][repo_link]["top_modules"].as_array_mut().unwrap().push(Value::from(module_name));
    }

    pub fn write_to_file(&self, filepath: &str) -> Result<()> {
        let toml_content = self.toml_doc.to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filepath)
            .expect("Failed to open vpm.toml");
        file.write_all(toml_content.as_bytes()).expect("Failed to write to vpm.toml");
        Ok(())
    }
}

pub fn add_dependency(git: &str, commit: Option<&str>) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    if !vpm_toml.get_dependencies().unwrap().contains_key(git) {
        vpm_toml.add_dependency(git, commit);
        vpm_toml.write_to_file("vpm.toml")?;
    }
    Ok(())
}

pub fn add_top_module(repo_link: &str, module_name: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.add_top_module(repo_link, module_name);
    vpm_toml.write_to_file("vpm.toml")?;
    Ok(())
}