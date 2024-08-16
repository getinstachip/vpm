use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    git: Option<String>,
    version: Option<String>,
    modules: Option<HashSet<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VpmToml {
    package: Package,
    dependencies: HashMap<String, Dependency>,
}

impl VpmToml {
    pub fn new() -> Self {
        Self {
            package: Package {
                name: "my-vpm-package".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["author-name <author-email>".to_string()],
                description: "My VPM package".to_string(),
                license: "My License".to_string(),
                repository: "https://github.com/<username>/<repository>".to_string(),
            },
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, package_name: &str, dependency: Dependency) {
        self.dependencies.insert(package_name.to_string(), dependency);
    }
}

fn read_toml() -> Result<VpmToml> {
    let contents = fs::read_to_string("./vpm.toml")?;
    let vpm_toml: VpmToml = toml::from_str(&contents)?;
    Ok(vpm_toml)
}

fn write_toml(vpm_toml: &VpmToml) -> Result<()> {
    let toml_string = toml::to_string(vpm_toml)?;
    fs::write("./vpm.toml", toml_string)?;
    Ok(())
}

pub fn add_dependency(package_name: &str, git: Option<&str>, version: Option<&str>, modules: Option<HashSet<String>>) -> Result<()> {
    let mut vpm_toml = if let Ok(toml) = read_toml() {
        toml
    } else {
        VpmToml::new()
    };

    let dependency = Dependency {
        git: git.map(String::from),
        version: version.map(String::from),
        modules,
    };

    vpm_toml.add_dependency(package_name, dependency);
    write_toml(&vpm_toml)
}
