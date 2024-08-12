use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, process::Command, process::Stdio};
use toml::{map::Map, Value};
use tree_sitter::{Parser, Node};
use std::fmt::Write as FmtWrite;

use crate::cmd::{Execute, Install};

const VPM_TOML: &str = "vpm.toml";
const VPM_LOCK: &str = "vpm.lock";
const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        let version = &self.version.clone().unwrap_or("0.1.0".to_string());
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' (vers:{}) from URL: '{}'", name, version, url);
            install_module_from_url(name, url, true)?;
            update_toml("modules", name, url, version)?;
            update_toml("repositories", name, url, version)?;
            update_lock("repositories", name, url, &get_commit_details(url)?[0], None)?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$")
                .unwrap()
                .is_match(arg)
            {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}' (vers:{})", url, version);
                install_repo_from_url(&url, "./vpm_modules/")?;
                update_toml("repositories", "", &url, version)?;
                update_lock("repositories", "", &url, &get_commit_details(&url)?[0], None)?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' (vers:{}) from standard library", name, version);
                install_module_from_url(&name, STD_LIB_URL, true)?;
                update_toml("modules", &name, STD_LIB_URL, version)?;
                // update_lock("modules", &name, STD_LIB_URL, commit_code)?;
            }
        } else {
            println!("Command not found!");
        }

        Ok(())
    }
}

fn update_toml(section_name: &str, module_name: &str, uri: &str, version: &str) -> Result<()> {
    if !PathBuf::from(VPM_TOML).exists() {
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(PathBuf::from(VPM_TOML))?;
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
        toml_section_map.insert(uri.to_string(), Value::String(version.to_string()));
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

pub fn install_module_from_url(module: &str, url: &str, sub: bool) -> Result<()> {
    let package_name = url
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default()
        .to_string();

    let mut visited_modules = HashSet::new();

    install_repo_from_url(url, "/tmp/")?;

    download_module(&format!("/tmp/{}", package_name), module, module, &package_name, url, &mut visited_modules, sub)?;

    fn download_module(dir: &str, module: &str, top_module: &str, package_name: &str, uri: &str, visited_modules: &mut HashSet<String>, sub: bool) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |name| name == module) {
                let mut file = fs::File::open(&path)?;
                if !sub {
                    let destination_dir = format!("./vpm_modules/{}", top_module);
                    fs::create_dir_all(&destination_dir)?;
                    let destination_path = format!("{}/{}", destination_dir, module);
                    fs::copy(&path, destination_path)?;
                    fs::remove_file(&path)?;
                    return Ok(());
                }
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let mut parser = Parser::new();
                parser
                    .set_language(tree_sitter_verilog::language())
                    .expect("Error loading Verilog grammar");

                if let Some(tree) = parser.parse(&contents, None) {
                    let root_node = tree.root_node();
                  
                    find_module_instantiations(
                        root_node,
                        package_name,
                        top_module,
                        &contents,
                        visited_modules,
                        module,
                        uri)?;
                  
                    let destination_dir = format!("./vpm_modules/{}", module);
                    fs::create_dir_all(&destination_dir)?;
                    let destination_path = format!("{}/{}", destination_dir, module);
                    fs::copy(&path, destination_path)?;
                    fs::remove_file(&path)?;

                    println!("Generating header files for {}", module);
                    fs::File::create(PathBuf::from(destination_dir).join(format!("{}h", module)))?.write_all(generate_headers(root_node, module, &contents)?.as_bytes())?;
                }

                return Ok(());
            } else if path.is_dir() {
                download_module(
                    path.to_str().unwrap_or_default(),
                    module,
                    top_module,
                    package_name,
                    uri,
                    visited_modules,
                    sub,
                )?;
            }
        }

        fn find_module_instantiations(
            root_node: Node,
            package_name: &str,
            top_module: &str,
            contents: &str,
            visited_modules: &mut HashSet<String>,
            root_mod_name: &str,
            uri: &str
        ) -> Result<()> {
            let mut cursor = root_node.walk();
            let mut dependencies: Vec<&str> = Vec::new();
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name: String = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                dependencies.push(module);
                                visited_modules.insert(module_name.clone());
                                download_module(
                                    &format!("/tmp/{}", package_name),
                                    &module_name,
                                    top_module,
                                    package_name,
                                    uri,
                                    visited_modules,
                                    true,
                                )?;
                            }
                        }
                    }
                }
                else {
                    find_module_instantiations(child, package_name, top_module, contents, visited_modules, root_mod_name, uri)?;
                }
            }
            
            update_lock("modules", root_mod_name, uri, &get_commit_details(uri)?[0], Some(&dependencies))?;
            Ok(())
        }

        Ok(())
    }

    fs::remove_dir_all(format!("/tmp/{}", package_name))?;

    Ok(())
}

pub fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default(),
    );

    fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
        Command::new("git")
            .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

        Ok(())
    }

    clone_repo(url, repo_path.to_str().unwrap_or_default())?;

    Ok(())
}

fn generate_headers(root_node: Node, module: &str, contents: &str) -> Result<String> {
    let mut header_content = format!("// Header for module {}\n\n", module);
    let guard_name = module.replace('.', "_").to_uppercase();

    header_content.push_str(&format!("`ifndef _{0}H_\n`define _{0}H_\n\n", guard_name));

    fn process_node(node: Node, contents: &str, header_content: &mut String) -> Result<()> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "parameter_declaration" | "local_parameter_declaration" | "default_nettype_compiler_directive" => {
                    let mut cursor_node = child.walk();
                    for node in child.children(&mut cursor_node) {
                        write!(header_content, "{} ", node.utf8_text(contents.as_bytes())?)?;
                    }
                    header_content.push('\n');
                }
                _ => {
                    process_node(child, contents, header_content)?;
                }
            }
        }
        Ok(())
    }

    process_node(root_node, contents, &mut header_content)?;

    header_content.push_str(&format!("\n`endif // _{}H_\n", guard_name));

    Ok(header_content)
}
