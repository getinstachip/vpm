use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, process::Command};
use tokio::runtime::Runtime;
use toml_edit::DocumentMut;
use tree_sitter::Parser;

use clust::messages::{
    ClaudeModel,
    MaxTokens,
    Message,
    MessagesRequestBody,
    SystemPrompt
};
use clust::Client;

use crate::cmd::{Execute, Install};
use crate::versions::versions::{read_file, update_dependencies_entry};

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        let version = &self.version.clone();
        let mut toml_doc = read_file(false)?;
        let mut lock_doc = read_file(true)?;

        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' (vers:{}) from URL: '{}'", name, version.clone().unwrap_or("".to_string()), url);
            install_module_from_url(name, url, version.as_deref(), &mut toml_doc, &mut lock_doc)?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$")
                .unwrap()
                .is_match(arg)
            {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}' (vers:{})", url, version.clone().unwrap_or("".to_string()));
                install_repo_from_url(&url, "./vpm_modules/", &mut toml_doc, &mut lock_doc)?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' (vers:{}) from standard library", name, version.clone().unwrap_or("".to_string()));
                install_module_from_url(&name, STD_LIB_URL, version.as_deref(), &mut toml_doc, &mut lock_doc)?;
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

fn get_commit_details(url: &str) -> Result<(Option<String>, Option<String>)> {
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

    Ok((Some(commit_code), Some(branch)))
}

fn install_module_from_url(module: &str, url: &str, version: Option<&str>, toml_doc: &mut DocumentMut, lock_doc: &mut DocumentMut) -> Result<()> {
    
    let package_name = name_from_url(url)?.to_string();

    let mut visited_modules = HashSet::new();

    install_repo_from_url(url, "/tmp/", toml_doc, lock_doc)?;

    download_module(&format!("/tmp/{}", package_name),
                    module,
                    &package_name,
                    url,
                    version,
                    &mut visited_modules,
                    toml_doc,
                    lock_doc)?;

    fn download_module(dir: &str,
                       module: &str,
                       package_name: &str,
                       uri: &str,
                       version: Option<&str>,
                       visited_modules: &mut HashSet<String>,
                       toml_doc: &mut DocumentMut,
                       lock_doc: &mut DocumentMut) -> Result<()> {
        
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
                        visited_modules,
                        module,
                        uri,
                        version,
                        vec![],
                        toml_doc,
                        lock_doc)?;
                  
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
                    uri,
                    version,
                    visited_modules,
                    toml_doc,
                    lock_doc)?;
            }
        }

        fn find_module_instantiations(root_node: tree_sitter::Node,
                                      package_name: &str,
                                      contents: &str,
                                      visited_modules: &mut HashSet<String>,
                                      root_mod_name: &str,
                                      uri: &str,
                                      version: Option<&str>,
                                      depends: Vec<String>,
                                      toml_doc: &mut DocumentMut,
                                      lock_doc: &mut DocumentMut) -> Result<Vec<String>> {

            let mut cursor = root_node.walk();
            let mut dependencies: Vec<String> = if depends.contains(&root_mod_name.to_string()) { vec![] } else { vec![root_mod_name.to_string()] };
            for child in root_node.children(&mut cursor) {
                if child.kind().contains("instantiation") {
                    if let Some(first_child) = child.child(0) {
                        if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                            let module_name: String = format!("{}.v", module);
                            if !visited_modules.contains(&module_name) {
                                dependencies.push(module.to_string());
                                visited_modules.insert(module_name.clone());
                                download_module(&format!("/tmp/{}", package_name),
                                                &module_name,
                                                package_name,
                                                uri,
                                                version,
                                                visited_modules,
                                                toml_doc,
                                                lock_doc)?;
                            }
                        }
                    }
                }
                dependencies.append(&mut find_module_instantiations(child,
                                                                    package_name,
                                                                    contents,
                                                                    visited_modules,
                                                                    root_mod_name,
                                                                    uri,
                                                                    version,
                                                                    dependencies.clone(),
                                                                    toml_doc,
                                                                    lock_doc)?);
            }
            
            let (branch, commit) = if version != Some("") {get_commit_details(uri)?} else {(Some("".to_string()), Some("".to_string()))};
            update_dependencies_entry(toml_doc,
                                      "dependencies",
                                      uri,
                                      version,
                                      Some(package_name),
                                      Some(dependencies.clone()),
                                      branch.as_deref(),
                                      commit.as_deref())?;
            update_dependencies_entry(lock_doc,
                                      "lock-dependencies",
                                      uri,
                                      version,
                                      Some(package_name),
                                      Some(dependencies.clone()),
                                      branch.as_deref(),
                                      commit.as_deref())?;
            Ok(dependencies.clone())
        }

        Ok(())
    }

    let rt = Runtime::new()?;
    rt.block_on(generate_docs(&format!("./vpm_modules/{}/", package_name), module))?;
    fs::remove_dir_all(format!("/tmp/{}", package_name))?;

    Ok(())
}

fn install_repo_from_url(url: &str, location: &str, toml_doc: &mut DocumentMut, lock_doc: &mut DocumentMut) -> Result<()> {
    let repo_path = PathBuf::from(location).join(name_from_url(url)?,);

    fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
        Command::new("git")
            .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
            .status()
            .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

        Ok(())
    }

    clone_repo(url, repo_path.to_str().unwrap_or_default())?;

    let (branch, commit) = get_commit_details(url)?;
    update_dependencies_entry(toml_doc,
                              "dependencies",
                              url, Some(""),
                              Some(""),
                              vec![].into(),
                              branch.as_deref(),
                              commit.as_deref())?;
    update_dependencies_entry(lock_doc,
                              "lock-dependencies",
                              url, Some(""),
                              Some(""),
                              vec![].into(),
                              branch.as_deref(),
                              commit.as_deref())?;

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
    let response_content = response.content;

    let parsed_response: serde_json::Value = serde_json::from_str(&response_content.to_string())?;
    let generated_text = parsed_response[0]["text"].as_str().unwrap_or("");

    let readme_path = PathBuf::from(dir).join("README.md");
    let mut readme_file = fs::File::create(&readme_path)?;

    writeln!(readme_file, "# {}", module)?;
    writeln!(readme_file)?;

    write!(readme_file, "{}", generated_text)?;

    println!("Documentation for {} written to {}", module, readme_path.display());

    Ok(())
}
