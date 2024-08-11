use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fmt::Write as FmtWrite;
use std::{fs, process::Command};

use anyhow::{Context, Result};
use regex::Regex;
use toml::{map::Map, Value};
use tree_sitter::{Parser, Node};
use tokio::runtime::Runtime;

use clust::messages::{
    ClaudeModel,
    MaxTokens,
    Message,
    MessagesRequestBody,
    SystemPrompt
};
use clust::Client;

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
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$")
                .unwrap()
                .is_match(arg)
            {
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
        } else {
            println!("Command not found!");
        }

        Ok(())
    }
}

fn add_dependency(section: &str, package: &str, version: &str) -> Result<()> {
    if !PathBuf::from(VPM_TOML).exists() {
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(PathBuf::from(VPM_TOML))?;
    }

    let content = fs::read_to_string(VPM_TOML)?;
    let mut toml_value: Value = content.parse()?;

    let table = toml_value.as_table_mut().unwrap();
    let section_map = table
        .entry(section.to_string())
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .unwrap();

    section_map.insert(package.to_string(), Value::String(version.to_string()));

    let new_content = toml::to_string(&toml_value)?;

    fs::write(VPM_TOML, new_content)?;

    Ok(())
}

fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = url
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default()
        .to_string();

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
                        visited_modules
                    )?;
                  
                    let destination_dir = format!("./vpm_modules/{}", top_module);
                    fs::create_dir_all(&destination_dir)?;
                    let destination_path = format!("{}/{}", destination_dir, module);
                    fs::copy(&path, destination_path)?;
                    fs::remove_file(&path)?;

                    println!("Generating header files for {}", module);
                    fs::File::create(PathBuf::from(destination_dir).join(format!("{}h", module)))?.write_all(generate_headers(root_node, module, &contents)?.as_bytes())?;

                    find_module_instantiations(root_node, package_name, top_module, &contents, visited_modules)?;
                }

                return Ok(());
            } else if path.is_dir() {
                download_module(
                    path.to_str().unwrap_or_default(),
                    module,
                    top_module,
                    package_name,
                    visited_modules,
                )?;
            }
        }

        fn find_module_instantiations(
            root_node: Node,
            package_name: &str,
            top_module: &str,
            contents: &str,
            visited_modules: &mut HashSet<String>,
        ) -> Result<()> {
            let mut cursor = root_node.walk();
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name: String = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                visited_modules.insert(module_name.clone());
                                download_module(
                                    &format!("/tmp/{}", package_name),
                                    &module_name,
                                    top_module,
                                    package_name,
                                    visited_modules,
                                )?;
                            }
                        }
                    }
                }
                else {
                    find_module_instantiations(child, package_name, top_module, contents, visited_modules)?;
                }
            }

            Ok(())
        }

        Ok(())
    }

    let rt = Runtime::new()?;
    rt.block_on(generate_docs(&format!("./vpm_modules/{}/", package_name), module))?;
    fs::remove_dir_all(format!("/tmp/{}", package_name))?;

    Ok(())
}

fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default(),
    );

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

async fn generate_docs(dir: &str, module: &str) -> Result<()> {
    println!("Generating Documentation for for {}, will be written to {}/README.md", module, dir);

    let mut file = fs::File::open(format!("{}/{}", dir, module))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let client = Client::from_env()?;
    let model = ClaudeModel::Claude35Sonnet20240620;
    let messages = vec![Message::user(
        format!(
            "Create a comprehensive Markdown documentation for the following Verilog module. Include an overview, module description, port list, parameters, and any important implementation details: {}",
            contents
        ),
    )];
    let max_tokens = MaxTokens::new(2048, model)?;
    let system_prompt = SystemPrompt::new("You are an expert Verilog engineer tasked with creating clear and detailed documentation.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    let response = client.create_a_message(request_body).await?;
    let generated_text = response.content.flatten_into_text().unwrap_or("Error generating docs");

    let readme_path = PathBuf::from(dir).join("README.md");
    let mut readme_file = fs::File::create(&readme_path)?;

    writeln!(readme_file, "```{}```", module)?;
    writeln!(readme_file)?;

    write!(readme_file, "{}", generated_text)?;

    println!("Documentation for {} written to {}", module, readme_path.display());

    Ok(())
}
