use std::collections::HashSet;
use walkdir::WalkDir;
use anyhow::Result;

use crate::toml::{VpmToml, Dependency};
use crate::lock::{Lockfile, Package as LockPackage};

pub fn resolve_dependencies() -> Result<()> {
    let vpm_toml = read_vpm_toml()?;
    let resolved_packages = resolve_packages(&vpm_toml)?;
    generate_lockfile(resolved_packages)
}

fn read_vpm_toml() -> Result<VpmToml> {
    let contents = std::fs::read_to_string("vpm.toml")?;
    let vpm_toml: VpmToml = toml::from_str(&contents)?;
    Ok(vpm_toml)
}

fn resolve_packages(vpm_toml: &VpmToml) -> Result<Vec<LockPackage>> {
    let mut resolved_packages = Vec::new();
    let mut visited = HashSet::new();

    for (name, dependency) in &vpm_toml.dependencies {
        if !visited.contains(name) {
            let package = resolve_package(name, dependency, &mut visited)?;
            resolved_packages.push(package);
        }
    }

    Ok(resolved_packages)
}

fn resolve_package(name: &str, dependency: &Dependency, visited: &mut HashSet<String>) -> Result<LockPackage> {
    visited.insert(name.to_string());

    let version = dependency.version.clone().unwrap_or_else(|| "0.1.0".to_string());
    let source = dependency.git.clone();

    let submodules: HashSet<String> = dependency.modules.iter()
        .filter_map(|dependency| get_submodules(dependency).ok())
        .flatten()
        .collect();

    Ok(LockPackage {
        name: name.to_string(),
        version,
        source,
        submodules: Some(submodules),
    })
}

fn generate_lockfile(packages: Vec<LockPackage>) -> Result<()> {
    let lockfile = Lockfile {
        version: 1,
        packages,
    };

    crate::lock::write_lockfile(&lockfile)
}

fn get_submodules(module: &str) -> Result<HashSet<String>> {
    let mut submodules = HashSet::new();
    let module_path = format!("./vpm_modules/{}", module);
    
    for entry in WalkDir::new(&module_path).into_iter().filter_map(Result::ok) {
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".v") {
                if let Ok(relative_path) = entry.path().strip_prefix(&module_path) {
                    let path_str = relative_path.to_string_lossy();
                    if !path_str.is_empty() {
                        submodules.insert(path_str.into_owned());
                    }
                }
            }
        }
    }
    
    Ok(submodules)
}
