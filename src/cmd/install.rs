use anyhow::{Context, Result};
use std::collections::HashSet;
use std::io::Read;
use std::{fs, process::Command};
use std::path::PathBuf;
use tree_sitter::Parser;
use chatgpt::prelude::*;
use regex::Regex;

use crate::cmd::{Execute, Install};

const STD_LIB_URL: &str = "https://github.com/vlang/v/tree/master/thirdparty/"; // edit to accept stdlib url

impl Execute for Install {
    async fn execute(&self) -> Result<()> {
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' from URL: '{}'", name, url);
            install_module_from_url(name, url)?;
            generate_docs(name, "./vpm_modules/").await?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap().is_match(arg) {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}'", url);
                install_repo_from_url(&url, "./vpm_modules/")?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' from standard library", name);
                install_module_from_url(&name, STD_LIB_URL)?;
            }
        }

        Ok(())
    }
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default().to_string())
}

fn install_module_from_url(url: &str, module: &str) -> Result<()> {
    let package_name = name_from_url(url)?;
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

async fn generate_docs(package_name: &str, location: &str) -> Result<()> {
    let package_path = PathBuf::from(location).join(package_name);

    let readme_path = package_path.join("README.md");
    let content = if readme_path.exists() {
        fs::read_to_string(readme_path)?
    } else {
        // If README.md doesn't exist, try to find a Verilog file
        let verilog_files: Vec<_> = fs::read_dir(&package_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "v" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        fs::read_to_string(&verilog_files[0])?
    };
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let openai_client = ChatGPT::new(key)?;
    let response = openai_client.send_message(content).await?;
    let content = response.message().content.clone();

    // Save the generated documentation to a README.md file
    let readme_path = package_path.join("README.md");
    fs::write(&readme_path, content)?;

    println!("Documentation saved to: {}", readme_path.display());

    Ok(())
}
