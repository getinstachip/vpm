use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependency {
    git: Option<String>,
    version: Option<String>,
    commit: Option<String>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    modules: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct VpmToml {
    package: Package,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<Dependency>,
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

impl Dependency {
    fn new() -> Self {
        Dependency {
            git: None,
            version: None,
            commit: None,
            modules: HashSet::new(),
        }
    }

    pub fn get_url(&self) -> Option<&str> {
        self.git.as_deref()
    }
}

impl Clone for Dependency {
    fn clone(&self) -> Self {
        Dependency {
            git: self.git.clone(),
            version: self.version.clone(),
            commit: self.commit.clone(),
            modules: self.modules.clone(),
        }
    }
}

impl VpmToml {
    fn add_dependency(&mut self, git: Option<&str>, version: Option<&str>, commit: Option<&str>) {
        let mut dependency = Dependency::new();
        dependency.git = git.map(String::from);
        dependency.version = version.map(String::from);
        dependency.commit = commit.map(String::from);
        self.dependencies.push(dependency);
    }

    fn add_module_to_dependency(&mut self, module: &str) {
        self.dependencies.last_mut().unwrap().modules.insert(module.to_string());
    }
}

pub fn init() -> Result<()> {
    let vpm_toml = VpmToml::default();
    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}

pub fn add_dependency(git: Option<&str>, version: Option<&str>, commit: Option<&str>) -> Result<()> {
    let mut vpm_toml = fs::read_to_string("vpm.toml")
        .map(|contents| toml::from_str(&contents))
        .unwrap_or_else(|_| Ok(VpmToml::default()))?;

    vpm_toml.add_dependency(git, version, commit);

    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}