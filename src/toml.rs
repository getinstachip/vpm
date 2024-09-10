use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use std::collections::HashSet;
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

    pub fn add_dependency(&mut self, git: &str) {
        self.toml_doc["dependencies"][git] = Item::Value(Value::Array(Array::new()));
    }

    pub fn add_top_module(&mut self, repo_link: &str, module_name: &str, commit: &str) {
        let array = self.toml_doc["dependencies"][repo_link].as_array_mut().unwrap();
        if !array.iter().any(|m| m.as_inline_table().unwrap().get("top_module").unwrap().as_str().unwrap() == module_name) {
            let new_entry = Value::InlineTable({
                let mut table = InlineTable::new();
                table.insert("top_module".to_string(), Value::from(module_name));
                table.insert("commit_hash".to_string(), Value::from(commit.to_string()));
                table
            });
            array.push(new_entry);
        }
    }

    pub fn remove_dependency(&mut self, git: &str) {
        if let Some(dependencies) = self.toml_doc["dependencies"].as_table_mut() {
            dependencies.remove(git);
        }
    }

    pub fn remove_top_module(&mut self, repo_link: &str, module_name: &str) {
    if let Some(dependencies) = self.toml_doc["dependencies"].as_table_mut() {
        if let Some(modules) = dependencies.get_mut(repo_link).and_then(|v| v.as_array_mut()) {
            modules.retain(|m| {
                if let Some(table) = m.as_inline_table() {
                    if let Some(top_module) = table.get("top_module").and_then(|v| v.as_str()) {
                        return top_module != module_name;
                    }
                }
                true
            });

            // If the array is empty after removal, remove the entire dependency
            if modules.is_empty() {
                dependencies.remove(repo_link);
            }
        }
    }
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

    pub fn get_repo_links(&self, module_name: &str) -> HashSet<String> {
        let mut repo_links = HashSet::new();
        if let Some(dependencies) = self.toml_doc["dependencies"].as_table() {
            for (repo_link, dependency) in dependencies.iter() {
                if let Some(top_modules) = dependency.as_array() {
                    if top_modules.iter().any(|m| m.as_inline_table().unwrap().get("top_module").unwrap().as_str().unwrap() == module_name) {
                        repo_links.insert(repo_link.to_string());
                    }
                }
            }
        }
        repo_links
    }
}

pub fn add_dependency(git: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    if !vpm_toml.get_dependencies().unwrap().contains_key(git) {
        vpm_toml.add_dependency(git);
        vpm_toml.write_to_file("vpm.toml")?;
    }
    Ok(())
}

pub fn add_top_module(repo_link: &str, module_path: &str, commit: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.add_top_module(repo_link, module_path, commit);
    vpm_toml.write_to_file("vpm.toml")?;
    Ok(())
}

fn remove_dependency(git: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.remove_dependency(git);
    vpm_toml.write_to_file("vpm.toml")?;
    Ok(())
}

pub fn remove_top_module(repo_link: &str, module_name: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.remove_top_module(repo_link, module_name);
    if let Some(dependencies) = vpm_toml.toml_doc["dependencies"].as_table() {
        if let Some(modules) = dependencies.get(repo_link).and_then(|v| v.as_array()) {
            if modules.is_empty() {
                remove_dependency(repo_link)?;
            }
        }
    }
    vpm_toml.write_to_file("vpm.toml")?;
    Ok(())
}

pub fn get_repo_links(module_name: &str) -> HashSet<String> {
    let vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.get_repo_links(module_name)
}