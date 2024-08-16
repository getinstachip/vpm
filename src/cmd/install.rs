use anyhow::{Context, Result};
use regex::Regex;
use std::path::PathBuf;
use std::{fs, process::Command};
use tree_sitter::{Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;
use std::collections::HashSet;

use crate::cmd::{Execute, Install};

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        if !PathBuf::from("./vpm_modules").exists() {
            fs::create_dir("./vpm_modules")?;
        }
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module '{}' from URL: '{}'", name, url);
            install_module_from_url(name, url)?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$")
                .unwrap()
                .is_match(arg)
            {
                let url = arg.to_string();
                println!("Installing repository from URL: '{}'", url);
                install_repo_from_url(&url, "./vpm_modules/")?;
            } else {
                let name = arg.to_string();
                println!("Installing module '{}' from standard library", name);
                install_module_from_url(&name, STD_LIB_URL)?;
            }
        } else {
            println!("Command not found!");
        }

        Ok(())
    }
}

fn name_from_url(url: &str) -> Result<String> {
    Ok(url
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or_default()
        .to_string())
}

pub fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url)?.to_string();

    install_repo_from_url(url, "/tmp/")?;

    let mut visited = HashSet::new();
    process_module(&package_name, module, &mut visited)?;

    fs::remove_dir_all(format!("/tmp/{}", package_name))?;

    Ok(())
}

fn process_module(package_name: &str, module: &str, visited: &mut HashSet<String>) -> Result<()> {
    dbg!(&module);
    visited.insert(module.replace(".v", ""));
    for entry in WalkDir::new(format!("/tmp/{}", package_name)) {
        let entry = entry?;
        if Some(module) == entry.file_name().to_str() {
            let path = entry.path();
            let contents = fs::read_to_string(path)?;
            let mut parser = Parser::new();
            parser
                .set_language(tree_sitter_verilog::language())
                .expect("Error loading Verilog grammar");

            let tree = parser
                .parse(&contents, None)
                .expect("Failed to parse the file");
        
            let root_node = tree.root_node();

            let header_content = generate_headers(root_node, &contents)?;
            fs::write(PathBuf::from("./vpm_modules").join(format!("{}h", module)), header_content)?;

            for submodule in get_submodules(root_node, &contents, visited)? {
                if !visited.contains(&submodule) {
                    process_module(package_name, &format!("{}.v", &submodule), visited)?;
                }
            }

            break;
        }
    }

    Ok(())
}

fn generate_headers(root_node: Node, contents: &str) -> Result<String> {
    let query = Query::new(
        tree_sitter_verilog::language(),
        "(module_declaration) @module
         (port_declaration) @port
         (parameter_declaration) @param",
    )
    .expect("Failed to create query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, root_node, contents.as_bytes());

    let mut header_content = String::new();

    for match_ in matches {
        for capture in match_.captures {
            let capture_text = &contents[capture.node.byte_range()];
            match capture.index {
                0 => {
                    header_content.push_str(&format!("// Module Declaration\n{}\n\n", capture_text))
                }
                1 => header_content.push_str(&format!("// Port Declaration\n{}\n\n", capture_text)),
                2 => header_content
                    .push_str(&format!("// Parameter Declaration\n{}\n\n", capture_text)),
                _ => {}
            }
        }
    }

    Ok(header_content)
}

fn get_submodules(root_node: Node, contents: &str, visited: &mut HashSet<String>) -> Result<Vec<String>> {
    let query = Query::new(
        tree_sitter_verilog::language(),
        "(module_or_generate_item 
            (module_instantiation 
                (simple_identifier) @module_submodule
            )
        )
        (module_or_generate_item 
            (udp_instantiation 
                (simple_identifier) @module_submodule
            )
        )"
    )
    .expect("Failed to create query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, root_node, contents.as_bytes());

    let mut submodules = Vec::new();

    for match_ in matches {
        for capture in match_.captures {
            if capture.index == 0 {
                let capture_text = &contents[capture.node.byte_range()];
                if !visited.contains(capture_text) {
                    submodules.push(capture_text.to_string());
                }
            }
        }
    }

    Ok(submodules)
}

fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = PathBuf::from(location).join(name_from_url(url)?);

    fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
        Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--single-branch",
                "--jobs",
                "4",
                url,
                repo_path,
            ])
            .status()
            .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

        Ok(())
    }

    clone_repo(url, repo_path.to_str().unwrap_or_default())?;

    Ok(())
}
