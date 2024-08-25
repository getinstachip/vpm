use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;

use crate::cmd::{Execute, Include};
use crate::toml::{add_dependency, add_top_module};

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap()
});

impl Execute for Include {
    fn execute(&self) -> Result<()> {
        // Check if vpm.toml exists and has [package] section
        // let vpm_toml_path = Path::new("vpm.toml");
        // if !vpm_toml_path.exists() || !fs::read_to_string(vpm_toml_path)
        //     .map(|contents| contents.contains("[package]"))
        //     .unwrap_or(false)
        // {
        //     crate::toml::init()?;
        //     println!("Initialized new vpm.toml file.");
        // }
        fs::create_dir_all("./vpm_modules")?;
        match (&self.url, &self.package_name) {
            (Some(url), Some(name)) => {
                println!("Including module '{}' from URL: '{}'", name, url);
                include_module_from_url(name, url);
                add_dependency(url, None);
                add_top_module(url, name)
            }
            (Some(url), None) | (None, Some(url)) if URL_REGEX.is_match(url) => {
                println!("Including repository from URL: '{}'", url);
                include_repo_from_url(url, "./vpm_modules/")?;
                add_dependency(url, None)
            }
            (None, Some(name)) => {
                println!("Including module '{}' from standard library", name);
                include_module_from_url(name, STD_LIB_URL);
                add_dependency(STD_LIB_URL, None)
            }
            _ => {
                println!("Command not found!");
                Ok(())
            }
        }
    }
}

fn name_from_url(url: &str) -> &str {
    url.rsplit('/').find(|&s| !s.is_empty()).unwrap_or_default()
}

pub fn include_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url);
    let tmp_path = PathBuf::from("/tmp").join(package_name);

    include_repo_from_url(url, "/tmp/")?;
    let module_name = module.strip_suffix(".v").or_else(|| module.strip_suffix(".sv")).unwrap_or(module);
    let destination = format!("./vpm_modules/{}", module_name);
    fs::create_dir_all(&destination)?;

    process_module(package_name, module, destination.to_owned(), &mut HashSet::new())?;

    fs::remove_dir_all(tmp_path)?;

    Ok(())
}

pub fn process_module(package_name: &str, module: &str, destination: String, visited: &mut HashSet<String>) -> Result<HashSet<String>> {
    let module_name = module.strip_suffix(".v").or_else(|| module.strip_suffix(".sv")).unwrap_or(module);
    if !visited.insert(module_name.to_string()) {
        return Ok(HashSet::new());
    }
    println!("Including module '{}'", module_name);
    let tmp_path = PathBuf::from("/tmp").join(package_name);
    for entry in WalkDir::new(&tmp_path).into_iter().filter_map(Result::ok) {
        if entry.file_name() == module || entry.file_name().to_str() == Some(&format!("{}.sv", module_name)) || entry.file_name().to_str() == Some(&format!("{}.v", module_name)) {
            let target_path = PathBuf::from(&destination).join(module_name);

            let extension = if entry.path().extension().and_then(|s| s.to_str()) == Some("sv") {
                "sv"
            } else {
                "v"
            };

            fs::copy(
                entry.path(),
                target_path.with_extension(extension),
            )?;

            let contents = fs::read_to_string(entry.path())?;
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_verilog::language())?;

            let tree = parser.parse(&contents, None).context("Failed to parse file")?;
            let root_node = tree.root_node();

            let header_content = generate_headers(root_node, &contents)?;
            fs::write(
                target_path.with_extension(if extension == "sv" { "svh" } else { "vh" }),
                header_content,
            )?;

            for submodule in get_submodules(root_node, &contents)? {
                if !visited.contains(&submodule) {
                    process_module(package_name, &format!("{}.{}", submodule, extension), destination.to_owned(), visited)?;
                }
            }

            break;
        }
    }

    Ok(visited.clone())
}

pub fn generate_headers(root_node: Node, contents: &str) -> Result<String> {
    static QUERY: Lazy<Query> = Lazy::new(|| {
        Query::new(
            tree_sitter_verilog::language(),
            "(module_declaration
                (module_header
                    (module_keyword)
                    (simple_identifier) @module_name)
                (module_nonansi_header
                    (parameter_port_list)? @params
                    (list_of_ports) @ports)
            )
            (module_declaration
                (module_header
                    (module_keyword)
                    (simple_identifier) @module_name)
                (module_ansi_header
                    (parameter_port_list)? @params
                    (list_of_port_declarations)? @ports)
            )",
        )
        .expect("Failed to create query")
    });

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&QUERY, root_node, contents.as_bytes());

    let mut header_content = String::new();

    for match_ in matches {
        let mut module_name = "";
        let mut params = "";
        let mut ports = "";

        for capture in match_.captures {
            let capture_text = &contents[capture.node.byte_range()];
            match capture.index {
                0 => module_name = capture_text,
                1 => params = capture_text,
                2 => ports = capture_text,
                _ => {}
            }
        }
        
        header_content.push_str(&format!(
            "module {} {}(\n{}\n{});\n\n// TODO: Add module implementation\n\nendmodule // {}\n\n",
            module_name,
            if params.is_empty() { "" } else { "#(\n" },
            params,
            ports,
            module_name
        ));
    }

    Ok(header_content)
}

pub fn get_submodules(root_node: Node, contents: &str) -> Result<HashSet<String>> {
    static QUERY: Lazy<Query> = Lazy::new(|| {
        Query::new(
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
            )",
        )
        .expect("Failed to create query")
    });

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&QUERY, root_node, contents.as_bytes());

    let mut submodules = HashSet::new();

    for match_ in matches {
        for capture in match_.captures {
            if capture.index == 0 {
                let capture_text = &contents[capture.node.byte_range()];
                submodules.insert(capture_text.to_string());
            }
        }
    }

    Ok(submodules)
}

pub fn include_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = Path::new(location).join(name_from_url(url));
    clone_repo(url, &repo_path)?;
    Ok(())
}

pub fn clone_repo(url: &str, repo_path: &Path) -> Result<()> {
    Command::new("git")
        .args([ "clone", "--depth", "1", "--single-branch", "--jobs", "4",
            url, repo_path.to_str().unwrap_or_default(),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;
    Ok(())
}