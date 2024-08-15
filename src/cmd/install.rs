use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, process::Command};
use tree_sitter::Parser;
use std::fmt::Write as FmtWrite;

use crate::cmd::{Execute, Install};

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        let version = &self.version.clone().unwrap_or("0.1.0".to_string());
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' (vers:{}) from URL: '{}'", name, version, url);
            install_module_from_url(name, url)?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$")
                .unwrap()
                .is_match(arg)
            {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}' (vers:{})", url, version);
                install_repo_from_url(&url, "./vpm_modules/")?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' (vers:{}) from standard library", name, version);
                install_module_from_url(&name, STD_LIB_URL)?;
            }
        } else {
            println!("Command not found!");
        }

        Ok(())
    }
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

pub fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url)?.to_string();

    let mut visited_modules = HashSet::new();

    install_repo_from_url(url, "/tmp/")?;

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
                parser
                    .set_language(tree_sitter_verilog::language())
                    .expect("Error loading Verilog grammar");

                if let Some(tree) = parser.parse(&contents, None) {
                    let root_node = tree.root_node();
                  
                    find_module_instantiations(
                        root_node,
                        package_name,
                        &contents,
                        visited_modules)?;
                  
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
                    package_name,
                    visited_modules,
                )?;
            }
        }

        fn find_module_instantiations(root_node: tree_sitter::Node, package_name: &str, contents: &str, visited_modules: &mut HashSet<String>) -> Result<()> {
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

    fs::remove_dir_all(format!("/tmp/{}", package_name))?;

    Ok(())
}

fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(name_from_url(url)?,);

    fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
        Command::new("git")
            .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
            .status()
            .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

        Ok(())
    }

    clone_repo(url, repo_path.to_str().unwrap_or_default())?;

    Ok(())
}

fn generate_headers(root_node: tree_sitter::Node, module: &str, contents: &str) -> Result<String> {
    let mut header_content = format!("// Header for module {}\n\n", module);
    let guard_name = module.replace('.', "_").to_uppercase();

    header_content.push_str(&format!("`ifndef _{0}H_\n`define _{0}H_\n\n", guard_name));

    fn process_node(node: tree_sitter::Node, contents: &str, header_content: &mut String) -> Result<()> {
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
