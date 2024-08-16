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
struct Dependency {
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
}

pub fn add_dependency(package_name: &str, git: Option<&str>, version: Option<&str>, module: Option<&str>) -> Result<()> {
    let mut vpm_toml = fs::read_to_string("vpm.toml")
        .map(|contents| toml::from_str(&contents))
        .unwrap_or_else(|_| Ok(VpmToml::default()))?;

    vpm_toml.add_dependency(package_name, git, version, module);

    fs::write("vpm.toml", toml::to_string(&vpm_toml)?)?;
    Ok(())
}
