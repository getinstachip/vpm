use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license: String,
    repository: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependency {
    git: Option<String>,
    version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    modules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct VpmToml {
    package: Package,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    dependencies: HashMap<String, Dependency>,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: "my-vpm-package".to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["<author-name> <author-email>".to_string()],
            description: "A vpm package".to_string(),
            license: "LicenseRef-LICENSE".to_string(),
            repository: "https://github.com/<author-name>/<package-name>".to_string(),
        }
    }
}

impl Dependency {
    fn new() -> Self {
        Dependency {
            git: None,
            version: None,
            modules: Vec::new(),
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
            modules: self.modules.clone(),
        }
    }
}

impl VpmToml {
    fn add_dependency(&mut self, name: &str, git: Option<&str>, version: Option<&str>, module: Option<&str>) {
        let dependency = self.dependencies.entry(name.to_string()).or_default();
        
        if let Some(git_url) = git {
            dependency.git = Some(git_url.to_string());
        }
        if let Some(ver) = version {
            dependency.version = Some(ver.to_string());
        }
        if let Some(mod_name) = module {
            dependency.modules.push(mod_name.to_string());
        }
    }

    fn get_dependency(&self, package_name: &str) -> Option<&Dependency> {
        self.dependencies.iter().for_each(|(name, dependency)| {
            println!("Package: {}", name);
            if let Some(git) = &dependency.git {
                println!("  Git: {}", git);
            }
            if let Some(version) = &dependency.version {
                println!("  Version: {}", version);
            }
            if !dependency.modules.is_empty() {
                println!("  Modules:");
                for module in &dependency.modules {
                    println!("    - {}", module);
                }
            }
            println!(); // Add a blank line between dependencies
        });
        self.dependencies.get(package_name)
    }

    fn remove_dependency(&mut self, package_name: &str) {
        self.dependencies.remove(package_name);
    }

    fn update_dependency(&mut self, package_name: &str, git: Option<&str>, version: Option<&str>, module: Option<&str>) {
        self.remove_dependency(package_name);
        self.add_dependency(package_name, git, version, module);
    }
}

pub fn add_dependency(package_name: &str, git: Option<&str>, version: Option<&str>, module: Option<&str>) -> Result<()> {
    let mut vpm_toml = fs::read_to_string("vpm.toml")
        .map(|contents| toml::from_str(&contents))
        .unwrap_or_else(|_| Ok(VpmToml::default()))?;

    vpm_toml.add_dependency(package_name, git, version, module);

    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}

pub fn get_dependency(package_name: &str) -> Option<Dependency> {
    let vpm_toml = fs::read_to_string("vpm.toml")
        .ok()
        .and_then(|contents| toml::from_str::<VpmToml>(&contents).ok())
        .unwrap_or_default();

    vpm_toml.get_dependency(package_name).cloned()
}

pub fn remove_dependency(package_name: &str) -> Result<()> {
    let mut vpm_toml = fs::read_to_string("vpm.toml")
        .map(|contents| toml::from_str(&contents))
        .unwrap_or_else(|_| Ok(VpmToml::default()))?;

    vpm_toml.remove_dependency(package_name);

    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}

pub fn update_dependency(package_name: &str, git: Option<&str>, version: Option<&str>, module: Option<&str>) -> Result<()> {
    let mut vpm_toml = fs::read_to_string("vpm.toml")
        .map(|contents| toml::from_str(&contents))
        .unwrap_or_else(|_| Ok(VpmToml::default()))?;

    vpm_toml.update_dependency(package_name, git, version, module);

    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}