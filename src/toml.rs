use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use std::collections::HashSet;
use toml::value::Value;
use toml::Table;
use toml::map::Map;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VpmToml {
    dependencies: Table,
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

impl Default for VpmToml {
    fn default() -> Self {
        VpmToml {
            dependencies: Table::new(),
        }
    }
}

impl VpmToml {    
    pub fn add_dependency(&mut self, git: &str, commit: Option<&str>) {
        let mut dependency = Table::new();
        dependency.insert("modules".to_string(), Value::Array(vec![]));
        dependency.insert("commit".to_string(), Value::String(commit.unwrap_or_default().to_string()));
        self.dependencies.insert(git.to_string(), Value::Table(dependency));
    }

    pub fn get_dependencies(&self) -> Table {
        self.dependencies.clone()
    }
}

pub fn init() -> Result<()> {
    let vpm_toml = VpmToml::default();
    let toml_string = toml::to_string(&vpm_toml)?;
    fs::write("vpm.toml", toml_string)?;
    Ok(())
}

pub fn add_dependency(git: &str, commit: Option<&str>) -> Result<()> {
    let toml_content = fs::read_to_string("vpm.toml")?;
    let mut vpm_toml: VpmToml = toml::from_str(&toml_content)?;
    vpm_toml.add_dependency(git, commit);
    let mut root = Table::new();
    root.insert("dependencies".to_string(), Value::Table(vpm_toml.get_dependencies()));
    let toml_string = format_custom_toml(&root);
    fs::write("vpm.toml", toml_string)?;
    Ok(())
}

fn format_custom_toml(root: &Map<String, Value>) -> String {
    let deps = root.get("dependencies").and_then(|v| v.as_table()).unwrap();
    let (key, value) = deps.iter().next().unwrap();
    let table = value.as_table().unwrap();
    
    let formatted_table = table.iter()
        .map(|(k, v)| format!("{}={}", k, format_value(v)))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[dependencies]\n\"{}\" = {{{}}}", key, formatted_table)
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(arr) => format!("[{}]", arr.iter()
            .map(|v| format_value(v))
            .collect::<Vec<_>>()
            .join(", ")),
        _ => value.to_string(),
    }
}