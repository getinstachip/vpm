use anyhow::{Context, Result};
use std::collections::HashSet;
use std::io::Read;
use std::{fs, process::Command};
use std::path::PathBuf;
use tree_sitter::Parser;
use regex::Regex;
use toml::{Value, map::Map};

use crate::cmd::{Execute, Install};

const VPM_TOML: &str = "vpm.toml";
const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' from URL: '{}'", name, url);
            install_module_from_url(name, url)?;
            add_dependency("repositories", url, "0.1.0")?;
            add_dependency("modules", name, "0.1.0")?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap().is_match(arg) {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}'", url);
                install_repo_from_url(&url, "./vpm_modules/")?;
                add_dependency("repositories", &url, "0.1.0")?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' from standard library", name);
                install_module_from_url(&name, STD_LIB_URL)?;
                add_dependency("modules", &name, "0.1.0")?;
            }
        }

        Ok(())
    }
}

fn add_dependency(section: &str, package: &str, version: &str) -> Result<()> {
    if !PathBuf::from(VPM_TOML).exists() {
        fs::OpenOptions::new().create_new(true).write(true).open(PathBuf::from(VPM_TOML))?;
    }

    let content = fs::read_to_string(VPM_TOML)?;
    let mut toml_value: Value = content.parse()?;

    let table = toml_value.as_table_mut().unwrap();
    let section_map = table.entry(section.to_string())
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .unwrap();

    section_map.insert(package.to_string(), Value::String(version.to_string()));

    let new_content = toml::to_string(&toml_value)?;

    fs::write(VPM_TOML, new_content)?;

    Ok(())
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url)?;
    let mut visited_modules = HashSet::new();

    install_repo_from_url(url, "/tmp/")?;
    download_module(&format!("/tmp/{}", package_name), module, module, &package_name, &mut visited_modules)?;

    fn download_module(dir: &str, module: &str, top_module: &str, package_name: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
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
                    find_module_instantiations(root_node, package_name, top_module, &contents, visited_modules)?;
                }

                let destination_dir = format!("./vpm_modules/{}", top_module);
                fs::create_dir_all(&destination_dir)?;
                let destination_path = format!("{}/{}", destination_dir, module);
                fs::copy(&path, destination_path)?;
                fs::remove_file(&path)?;

                return Ok(())
            } else if path.is_dir() {
                download_module(path.to_str().unwrap_or_default(), module, top_module, package_name, visited_modules)?;
            }
        }

        fn find_module_instantiations(root_node: tree_sitter::Node, package_name: &str, top_module: &str, contents: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
            let mut cursor = root_node.walk();
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name: String = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                visited_modules.insert(module_name.clone());
                                download_module(&format!("/tmp/{}", package_name), &module_name, top_module, package_name, visited_modules)?;
                            }
                        }
                    }
                }
                find_module_instantiations(child, package_name, top_module, contents, visited_modules)?;
            }

            Ok(())
        }

        Ok(())
    }

    fs::remove_dir_all(format!("/tmp/{}", name_from_url(url)?))?;
    Ok(())
}

fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default()
    );

    clone_repo(url, repo_path.to_str().unwrap_or_default())?;

    Ok(())
}

fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
    Command::new("git")
    .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
    .status()
    .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}
