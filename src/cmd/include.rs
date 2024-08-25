use std::collections::HashSet;

use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;

use crate::cmd::{Execute, Include};
use crate::toml::add_dependency;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://github\.com/[\w-]+/[\w.-]+(?:/)?$").unwrap()
});

impl Execute for Include {
    fn execute(&self) -> Result<()> {
        fs::create_dir_all("./vpm_modules")?;
        println!("Including repository from URL: '{}'", self.url);
        include_repo_from_url(&self.url, "./vpm_modules/")?;
        // add_dependency(name_from_url(&self.url), Some(&self.url), None, None)?;
           
        // Prompt for specific module path
        println!("Enter the specific module path you want to include:");
        let mut module_path = String::new();
        std::io::stdin().read_line(&mut module_path)?;
        let module_path = module_path.trim();
        
        if !module_path.is_empty() {
            println!("Including module '{}' from URL: '{}'", module_path, self.url);
            include_module_from_url(module_path, &self.url)?;
        }
        
        Ok(())
    }
}

fn name_from_url(url: &str) -> &str {
    url.rsplit('/').find(|&s| !s.is_empty()).unwrap_or_default()
}

fn is_full_filepath(path: &str) -> bool {
    // Check if the path contains directory separators
    path.contains('/') || path.contains('\\')
}
fn filepath_to_direntry(filepath: &str) -> Option<walkdir::DirEntry> {
    let path = std::path::Path::new(filepath);
    let parent = path.parent()?;
    
    WalkDir::new(parent)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .find(|entry| entry.path() == path)
}


pub fn include_module_from_url(module_path: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url);
    let tmp_path = PathBuf::from("/tmp").join(package_name);

    include_repo_from_url(url, "/tmp/")?;
    let module_name = module_path.split('/').last().unwrap_or(module_path);
    let module_name = module_name.strip_suffix(".v").or_else(|| module_name.strip_suffix(".sv")).unwrap_or(module_name);
    println!("Processing module: {}", module_name);
    let destination = format!("./vpm_modules/{}", module_name);
    fs::create_dir_all(&destination)?;

    process_module(package_name, module_path, destination.to_owned(), &mut HashSet::new(), true)?;
    add_dependency(package_name, Some(url), None, Some(module_path))?;

    fs::remove_dir_all(tmp_path)?;

    Ok(())
}

fn process_file(file_path: &Path, target_path: &Path, extension: &str, visited: &mut HashSet<String>, package_name: &str, destination: &str) -> Result<()> {
    if !file_path.exists() {
        println!("File not found: {}", file_path.display());
        return Ok(());
    }
    println!("Processing file: {} (Is full filepath: {})", file_path.display(), is_full_filepath(&file_path.to_string_lossy()));
    println!("Target path: {}", target_path.display());
    fs::copy(
        &file_path,
        target_path.with_extension(extension),
    )?;

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())?;

    let contents = fs::read_to_string(file_path)?;

    let tree = parser.parse(&contents, None).context("Failed to parse file")?;
    let root_node = tree.root_node();

    let header_content = generate_headers(root_node, &contents)?;
    fs::write(
        target_path.with_extension(if extension == "sv" { "svh" } else { "vh" }),
        header_content,
    )?;

    for submodule in get_submodules(root_node, &contents)? {
        if !visited.contains(&submodule) {
            println!("Processing submodule '{}'", submodule);
            process_module(package_name, (submodule + "." + extension).as_str(), destination.to_owned(), visited, false)?;
        }
    }
    Ok(())
}

pub fn process_module(package_name: &str, module_path: &str, destination: String, visited: &mut HashSet<String>, is_full_filepath: bool) -> Result<HashSet<String>> {
    let module_name = module_path.split('/').last().unwrap_or(module_path);
    let module_name = module_name.strip_suffix(".v").or_else(|| module_name.strip_suffix(".sv")).unwrap_or(module_name);
    if !visited.insert(module_name.to_string()) {
        return Ok(HashSet::new());
    }

    let tmp_path = PathBuf::from("/tmp").join(package_name);
    let file_path = tmp_path.join(module_path);

    let target_path = PathBuf::from(&destination).join(module_name);

    let extension = if file_path.extension().and_then(|s| s.to_str()) == Some("sv") {
        "sv"
    } else {
        "v"
    };

    println!("Including module '{}'", module_name);

    if is_full_filepath {
        process_file(&file_path, &target_path, extension, visited, package_name, &destination)?;
    } else {
        for entry in WalkDir::new(&tmp_path).into_iter().filter_map(Result::ok) {
            if entry.file_name().to_str() == Some(&format!("{}.sv", module_name)) || entry.file_name().to_str() == Some(&format!("{}.v", module_name)) {
                // let file_path = Path::new(entry.file_name().to_str().unwrap());
                // println!("Processing file: {}", file_path.display());
                process_file(entry.path(), &target_path, extension, visited, package_name, &destination)?;
            }
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