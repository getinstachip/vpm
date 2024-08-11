use anyhow::{Context, Result};
use std::collections::HashSet;
use std::io::Read;
use std::{fs, process::Command};
use std::path::PathBuf;
use tree_sitter::Parser;
use regex::Regex;
use toml::{Value, map::Map};

use crate::cmd::{Execute, Install};

// #[path ="../versions.rs"]
// mod versions;

const VPM_TOML: &str = "vpm.toml";
const VPM_LOCK: &str = "vpm.lock";
const STD_LIB_URL: &str = "https://github.com/vlang/v/tree/master/thirdparty/"; // edit to accept stdlib url

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        let version = &self.version.clone().unwrap_or("0.1.0".to_string());
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' (vers:{}) from URL: '{}'", name, version, url);
            install_module_from_url(name, url)?;
            update_toml("modules", name, url, version)?;
            update_toml("repositories", name, url, version)?;
            // update_lock("modules", name, url, commit_code)?;
            update_lock("repositories", name, url, &get_commit_details(url)?[0], None)?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap().is_match(arg) {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}' (vers:{})", url, version);
                install_repo_from_url(&url, "./vpm_modules/")?;
                update_toml("repositories", "", &url, version)?;
                update_lock("repositories", "", &url, &get_commit_details(&url)?[0], None)?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' (vers:{}) from standard library", name, version);
                install_module_from_url(&name, STD_LIB_URL)?;
                update_toml("modules", &name, STD_LIB_URL, version)?;
                // update_lock("modules", &name, STD_LIB_URL, commit_code)?;
            }
        }

        Ok(())
    }
}

fn update_toml(section_name: &str, module_name: &str, uri: &str, version: &str) -> Result<()> {
    if !PathBuf::from(VPM_TOML).exists() {
        fs::OpenOptions::new().create_new(true).write(true).open(PathBuf::from(VPM_TOML))?;
    }
    let mut toml_value: Value = fs::read_to_string(VPM_TOML)?.parse()?;
    let toml_table = toml_value.as_table_mut().unwrap();
    let toml_section_map = toml_table.entry(section_name.to_string())
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .unwrap();

    if section_name == "modules" {
        let mut toml_entry_table = Map::new();
        toml_entry_table.insert("version".to_string(), Value::String(version.to_string()));
        toml_entry_table.insert("uri".to_string(), Value::String(uri.to_string()));
        toml_entry_table.insert("branch".to_string(), Value::String(get_commit_details(uri)?[1].to_string()));
        toml_section_map.insert(module_name.to_string(), Value::Table(toml_entry_table));
    } else if section_name == "repositories" {
        toml_section_map.insert(module_name.to_string(), Value::String(uri.to_string()));
    }

    let new_toml_content = toml::to_string(&toml_value)?;
    fs::write(VPM_TOML, new_toml_content)?;

    Ok(())
}

fn update_lock(section_name: &str, root_module: &str, repo_uri: &str, commit_code: &str, dependecies: Option<&Vec<&str>>) -> Result<()> {
    if !PathBuf::from(VPM_LOCK).exists() {
        fs::OpenOptions::new().create_new(true).write(true).open(PathBuf::from(VPM_LOCK))?;
    }
    let mut lock_value: Value = fs::read_to_string(VPM_LOCK)?.parse()?;
    let lock_table = lock_value.as_table_mut().unwrap();
    let lock_section_map = lock_table.entry(section_name.to_string())
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .unwrap();

    if section_name == "modules" {
        let mut values = dependecies.unwrap_or(&Vec::new()).iter().map(|x| x.to_string()).collect::<Vec<String>>();
        values.insert(0, repo_uri.to_string());
        lock_section_map.insert(root_module.to_string(), Value::Array(values.iter().map(|x| Value::String(x.to_string())).collect()));
    } else if section_name == "repositories" {
        lock_section_map.insert(repo_uri.to_string(), Value::String(commit_code.to_string()));
    }

    let new_lock_content = toml::to_string(&lock_value)?;
    fs::write(VPM_LOCK, new_lock_content)?;

    Ok(())
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

fn get_commit_details(url: &str) -> Result<Vec<String>> {
    let commit_code = Command::new("git")
        .args(["ls-remote", "--refs", url])
        .output()
        .with_context(|| format!("Failed to get commit code from URL: '{}'", url))?;
    let commit_code = String::from_utf8(commit_code.stdout)?;
    let commit_code = commit_code.split_whitespace().next().unwrap_or_default().to_string();

    let branch = Command::new("git")
        .args(["ls-remote", "--refs", url])
        .output()
        .with_context(|| format!("Failed to get branch from URL: '{}'", url))?;
    let branch = String::from_utf8(branch.stdout)?;
    let branch = branch.split_whitespace().nth(1).unwrap_or_default().to_string();

    Ok(vec![commit_code, branch])
}

fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url).unwrap();
    let mut visited_modules = HashSet::new();

    install_repo_from_url(url, "/tmp/").unwrap();
    download_module(&format!("/tmp/{}", package_name), module, &package_name, url, &mut visited_modules).unwrap();

    fn download_module(dir: &str, module: &str, package_name: &str, uri: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
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
                    find_module_instantiations(root_node, package_name, &contents, visited_modules, module, uri)?;
                }

                let destination_dir = format!("./vpm_modules/{}", package_name);
                fs::create_dir_all(&destination_dir)?;
                let destination_path = format!("{}/{}", destination_dir, module);
                fs::copy(&path, destination_path)?;
                fs::remove_file(&path)?;

                return Ok(())
            } else if path.is_dir() {
                download_module(path.to_str().unwrap_or_default(), module, package_name, uri, visited_modules)?;
            }
        }

        fn find_module_instantiations(root_node: tree_sitter::Node, package_name: &str, contents: &str, visited_modules: &mut HashSet<String>, root_mod_name: &str, uri: &str) -> Result<()> {
            let mut cursor = root_node.walk();
            let mut dependencies: Vec<&str> = Vec::new();
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                dependencies.push(module);
                                visited_modules.insert(module_name.clone());
                                download_module(&format!("/tmp/{}", package_name), &module_name, package_name, uri, visited_modules)?;
                            }
                        }
                    }
                }
                find_module_instantiations(child, package_name, contents, visited_modules, root_mod_name, uri)?;
            }
            
            update_lock("modules", root_mod_name, uri, &get_commit_details(uri)?[0], Some(&dependencies))?;
            Ok(())
        }

        Ok(())
    }

    fs::remove_dir_all(format!("/tmp/{}", name_from_url(url).unwrap())).unwrap();
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
