use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use std::collections::HashSet;
use toml::value::Value;
use toml::Table;
use toml::map::Map;
use cargo_lock::Lockfile;
use std::collections::BTreeMap;
use serde_json::from_value;


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
    toml_value: Value,
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
        let mut toml_content = fs::read_to_string(filepath).unwrap();
        toml_content = parse_and_format_toml(&toml_content);
        toml_content = add_package_info(&toml_content);
        Self {
            toml_value: toml::from_str(&toml_content).unwrap()
        }
    }

    pub fn get_dependencies(&self) -> Option<&Table> {
        self.toml_value.as_table().unwrap().get("dependencies").unwrap().as_table()
    }

    pub fn add_dependency(&mut self, git: &str, commit: Option<&str>) {
        let mut dependency = Table::new();
        dependency.insert("top_modules".to_string(), Value::Array(Vec::new()));
        dependency.insert("commit".to_string(), Value::String(commit.unwrap_or_default().to_string()));
        if let Some(dependencies) = self.toml_value.get_mut("dependencies") {
            dependencies.as_table_mut().unwrap().insert(
                git.to_string(),
                Value::Table(dependency)
            );
        } else {
            let mut dependencies = toml::value::Table::new();
            dependencies.insert(
                git.to_string(),
                Value::Table(dependency)
            );
            self.toml_value.as_table_mut().unwrap().insert("dependencies".to_string(), Value::Table(dependencies));
        }
    }

    pub fn add_top_module(&mut self, repo_link: &str, module_name: &str) {
        if let Some(dependencies) = self.toml_value.get_mut("dependencies") {
            if let Some(dependency) = dependencies.as_table_mut().unwrap().get_mut(repo_link) {
                println!("Dependency: {}", dependency);
                if let Some(top_modules) = dependency.get_mut("top_modules") {
                    println!("Top modules: {}", top_modules);
                    if let Some(array) = top_modules.as_array_mut() {
                        array.push(Value::String(module_name.to_string()));
                    }
                    println!("Top modules: {}", top_modules);
                } else {
                    dependency.as_table_mut().unwrap().insert("top_modules".to_string(), Value::Array(vec![Value::String(module_name.to_string())]));
                }
                println!("Dependency: {}", dependency);
            }
        }
    }

    pub fn get_package_info(&self) -> Option<&Table> {
        self.toml_value.as_table().unwrap().get("package").and_then(|v| v.as_table())
    }

    pub fn package_to_string(&self) -> String {
        if let Some(package) = self.get_package_info() {
            let formatted_package = package.iter()
                .map(|(k, v)| format!("{} = {}", k, format_value(v)))
                .collect::<Vec<_>>()
                .join("\n");
            format!("[package]\n{}", formatted_package)
        } else {
            println!("Error: No package section found in vpm.toml");
            "".to_string()
        }
    }

    pub fn all_deps_to_string(&self) -> String {
        let deps = self.get_dependencies();
        
        let mut formatted_deps = Vec::new();
        for (dep_key, dep_value) in deps.unwrap().iter() {
            let formatted_table = dep_value.as_table().unwrap().iter()
                .filter_map(|(k, v)| {
                    match v {
                        Value::Array(arr) if arr.is_empty() => None,
                        Value::String(s) if s.is_empty() => None,
                        _ => Some(format!("{}={}", k, format_value(v)))
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            formatted_deps.push(format!("\"{}\" = {{{}}}", dep_key, formatted_table));
        }
        let formatted_deps_str = formatted_deps.join("\n");
        
        format!("\n[dependencies]\n{}", formatted_deps_str)
    }

    pub fn to_string(&self) -> String {
        let toml_content = self.all_deps_to_string();
        let package_content = self.package_to_string();
        format!("{}\n{}", package_content, toml_content)
    }

    pub fn generate_lockfile_content(&self) -> String {
        let dependencies = self.get_dependencies();
        let mut lockfile_content = String::new();

        if let Some(deps) = dependencies {
            for (dep_url, dep_info) in deps {
                lockfile_content.push_str(&format!("[[package]]\n"));
                lockfile_content.push_str(&format!("source = \"{}\"\n", dep_url));
                
                if let Some(table) = dep_info.as_table() {
                    if let Some(commit) = table.get("commit").and_then(|v| v.as_str()) {
                        lockfile_content.push_str(&format!("version = \"{}\"\n", commit));
                    }
                    if let Some(top_modules) = table.get("top_modules").and_then(|v| v.as_array()) {
                        lockfile_content.push_str("modules = [");
                        let modules: Vec<String> = top_modules
                            .iter()
                            .filter_map(|m| m.as_str().map(|s| format!("\"{}\"", s)))
                            .collect();
                        lockfile_content.push_str(&modules.join(", "));
                        lockfile_content.push_str("]\n");
                    }
                }

                lockfile_content.push_str("\n");
            }
        }

        lockfile_content
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(arr) => {
            let formatted = arr.iter()
                .map(|v| match v {
                    Value::String(s) if s.is_empty() => "".to_string(),
                    _ => format_value(v)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", formatted)
        },
        _ => value.to_string(),
    }
}

pub fn add_dependency(git: &str, commit: Option<&str>) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    if let Some(dependencies) = vpm_toml.get_dependencies() {
        if !dependencies.contains_key(git) {
            vpm_toml.add_dependency(git, commit);
            let toml_string = vpm_toml.to_string();
            fs::write("vpm.toml", toml_string)?;
        } else {
            println!("Dependency '{}' already exists in vpm.toml", git);
        }
    } else {
        vpm_toml.add_dependency(git, commit);
        let toml_string = vpm_toml.to_string();
        fs::write("vpm.toml", toml_string)?;
    }
    Ok(())
}

pub fn add_top_module(repo_link: &str, module_name: &str) -> Result<()> {
    let mut vpm_toml = VpmToml::from("vpm.toml");
    vpm_toml.add_top_module(repo_link, module_name);
    let toml_string = vpm_toml.to_string();
    println!("TOML: {}", toml_string);
    fs::write("vpm.toml", toml_string)?;
    Ok(())
}

pub fn generate_lockfile() -> Result<()> {
    let vpm_toml = VpmToml::from("vpm.toml");
    let lockfile_content = vpm_toml.generate_lockfile_content();
    fs::write("vpm.lock", lockfile_content)?;
    Ok(())
}

fn add_package_info(input: &str) -> String {
    let mut package = Package::default();
    let mut in_package_section = false;
    let mut package_found = false;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package_section = true;
            package_found = true;
            continue;
        }
        if in_package_section && trimmed.starts_with('[') {
            break;
        }
        if in_package_section && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().trim_matches('"');
                match key {
                    "name" => package.name = value.to_string(),
                    "version" => package.version = value.to_string(),
                    "authors" => package.authors = value.split(',').map(|s| s.trim().to_string()).collect(),
                    "description" => package.description = value.to_string(),
                    "license" => package.license = value.to_string(),
                    _ => {}
                }
            }
        }
    }

    let package_to_use = if package_found { package } else { Package::default() };
    let mut root = toml::Table::new();
    if let Ok(existing_toml) = toml::from_str::<toml::Table>(input) {
        root = existing_toml;
    }
    if let Some(toml::Value::Table(package_table)) = root.get_mut("package") {
        let package_value = toml::to_string(&package_to_use).unwrap_or_default().parse::<toml::Value>().unwrap();
        if let toml::Value::Table(new_package_table) = package_value {
            package_table.extend(new_package_table);
        }
    } else {
        root.insert("package".to_string(), toml::Value::Table(toml::to_string(&package_to_use).unwrap_or_default().parse().unwrap_or_default()));
    }
    toml::to_string(&root).unwrap_or_else(|_| "".to_string())
}

fn parse_and_format_toml(input: &str) -> String {
    let mut root = Map::new();
    let mut current_section = String::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section = trimmed[1..trimmed.len()-1].to_string();
            root.insert(current_section.clone(), Value::Table(Map::new()));
        } else if !current_section.is_empty() && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches('"');
                let value = parts[1].trim();

                if value.starts_with('{') && value.ends_with('}') {
                    let inner_map = parse_inline_table(value);
                    if let Some(Value::Table(section_map)) = root.get_mut(&current_section) {
                        section_map.insert(key.to_string(), Value::Table(inner_map));
                    }
                }
            }
        }
    }

    toml::to_string(&root).unwrap_or_else(|_| "Error formatting TOML".to_string())
}

fn parse_inline_table(s: &str) -> Map<String, Value> {
    let mut map = Map::new();
    let inner = &s[1..s.len()-1];
    for pair in inner.split(',') {
        let kv: Vec<&str> = pair.splitn(2, '=').collect();
        if kv.len() == 2 {
            let k = kv[0].trim();
            let v = kv[1].trim();
            
            if k == "top_modules" && v.starts_with('[') && v.ends_with(']') {
                let top_modules: Vec<Value> = v[1..v.len()-1]
                    .split(',')
                    .map(|m| Value::String(m.trim().trim_matches('"').to_string()))
                    .collect();
                map.insert(k.to_string(), Value::Array(top_modules));
            } else {
                map.insert(k.to_string(), Value::String(v.trim_matches('"').to_string()));
            }
        }
    }
    map
}