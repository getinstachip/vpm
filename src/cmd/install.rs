use anyhow::{Context, Result};
use std::collections::HashSet;
use std::io::Read;
use std::{fs, process::Command};
use std::path::PathBuf;
use tree_sitter::Parser;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        const LOCATION: &str = "./vpm_modules";

        if let (Some(url), Some(module)) = (&self.url, &self.package_name) {
            install_module_from_url(url, module)?;
        } else if let Some(url) = &self.url {
            install_from_url(url, LOCATION)?;
        } else if let Some(_name) = &self.package_name {
            // TODO: Add package install logic
        }

        Ok(())
    }
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

fn install_module_from_url(url: &String, module: &str) -> Result<()> {
    let package_name = name_from_url(url)?;
    let mut visited_modules = HashSet::new();

    install_from_url(url, "/tmp/")?;
    download_module(&format!("/tmp/{}", package_name), module, &package_name, &mut visited_modules)?;

    fn download_module(dir: &str, module: &str, package_name: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |name| name == module) {
                let mut file = fs::File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let mut parser = Parser::new();
                parser.set_language(tree_sitter_verilog::language()).expect("Error loading Verilog grammar");

                if let Some(tree) = parser.parse(&contents, None) {
                    let root_node = tree.root_node();
                    find_module_instantiations(root_node, package_name, &contents, visited_modules)?;
                }

                let destination_dir = format!("./vpm_modules/{}", package_name);
                fs::create_dir_all(&destination_dir)?;
                let destination_path = format!("{}/{}", destination_dir, module);
                fs::copy(&path, destination_path)?;
                fs::remove_file(&path)?;

                return Ok(())
            } else if path.is_dir() {
                download_module(path.to_str().unwrap_or_default(), module, package_name, visited_modules)?;
            }
        }

        fn find_module_instantiations(root_node: tree_sitter::Node, package_name: &str, contents: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
            let mut cursor = root_node.walk();
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                visited_modules.insert(module_name.clone());
                                download_module(&format!("/tmp/{}", package_name), &module_name, package_name, visited_modules)?;
                            }
                        }
                    }
                }
                find_module_instantiations(child, package_name, contents, visited_modules)?;
            }

            Ok(())
        }

        Ok(())
    }

    fs::remove_dir_all(format!("/tmp/{}", name_from_url(url)?))?;
    Ok(())
}

fn install_from_url(url: &String, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        name_from_url(url).with_context(|| format!("Failed to parse URL: '{}'", url))?
    );

    Command::new("git")
        .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path.to_str().unwrap_or_default()])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}
