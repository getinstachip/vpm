use std::collections::HashSet;

use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use skim::{prelude::SkimOptionsBuilder, Skim, prelude::SkimItemReader};
use crate::cmd::{Execute, Include};
use crate::toml::add_dependency;
use std::io::Cursor;
use skim::SkimItem;
use std::sync::Arc;
use std::borrow::Cow;
use walkdir::{DirEntry, WalkDir};

use crate::cmd::{Execute, Include};
use crate::toml::{add_dependency, add_top_module, generate_lockfile};

struct Item {
    text: String,
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }
}

impl Execute for Include {
    fn execute(&self) -> Result<()> {
        fs::create_dir_all("./vpm_modules")?;
        println!("Including repository from URL: '{}'", self.url);
        let repo_name = name_from_url(&self.url);
        let tmp_path = PathBuf::from("/tmp").join(repo_name);
        include_repo_from_url(&self.url, "/tmp/")?;

        let files = get_files(&tmp_path.to_str().unwrap_or_default());

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(true)
            .build()
            .unwrap();

        let items: Vec<Arc<dyn SkimItem>> = files
            .into_iter()
            .map(|file| Arc::new(Item { text: file }) as Arc<dyn SkimItem>)
            .collect();

        let selected_items = Skim::run_with(
            &options,
            Some(SkimItemReader::default().of_bufread(Cursor::new(
                items
                    .iter()
                    .map(|item| {
                        let text = item.text().into_owned();
                        text.strip_prefix(&tmp_path.to_string_lossy().as_ref()).unwrap_or(&text).trim_start_matches('/').to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )))
        )
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

        let has_selected_items = !selected_items.is_empty();

        for item in &selected_items {
            let item_text = item.text();
            let displayed_path = item_text.strip_prefix(tmp_path.to_string_lossy().as_ref()).unwrap_or(&item_text).trim_start_matches('/');
            println!("Including module: {}", displayed_path);
            
            let full_path = tmp_path.join(displayed_path);
            let module_path = full_path.strip_prefix(&tmp_path).unwrap_or(&full_path).to_str().unwrap().trim_start_matches('/');
            
            include_module_from_url(module_path, &self.url)?;
        }

        if !has_selected_items {
            println!("No modules selected. Including entire repository.");
            include_repo_from_url(&self.url, "./vpm_modules/")?;
        }

        // add_dependency(name_from_url(&self.url), Some(&self.url), None, None)?;
        fs::remove_dir_all(tmp_path)?;
        Ok(())
    }
}

fn get_files(directory: &str) -> Vec<String> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.file_type().is_file() {
                    Some(e.path().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect()
}

fn name_from_url(url: &str) -> &str {
    url.rsplit('/').find(|&s| !s.is_empty()).unwrap_or_default()
}

fn is_full_filepath(path: &str) -> bool {
    // Check if the path contains directory separators
    path.contains('/') || path.contains('\\')
}

pub fn include_module_from_url(module_path: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url);
    let module_name = module_path.split('/').last().unwrap_or(module_path);
    let module_name = module_name.strip_suffix(".v").or_else(|| module_name.strip_suffix(".sv")).unwrap_or(module_name);
    println!("Processing module: {}", module_name);
    let destination = format!("./vpm_modules/{}", module_name);
    fs::create_dir_all(&destination)?;
    fs::create_dir_all(format!("{}/dependencies", &destination))?;

    process_module(package_name, module, destination.to_owned(), &mut HashSet::new(), url, true)?;

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


pub fn process_module(package_name: &str, module: &str, destination: String, visited: &mut HashSet<String>, url: &str, is_top_module: bool) -> Result<HashSet<String>> {
    let module_name = module.strip_suffix(".v").or_else(|| module.strip_suffix(".sv")).unwrap_or(module);
    let module_with_ext = if module.ends_with(".v") || module.ends_with(".sv") {
        module.to_string()
    } else {
        format!("{}.v", module)
    };
    if !visited.insert(module_with_ext.clone()) {
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
        let mut matching_entries = Vec::new();
        for entry in WalkDir::new(&tmp_path).into_iter().filter_map(Result::ok) {
            if entry.file_name().to_str() == Some(&format!("{}.sv", module_name)) || entry.file_name().to_str() == Some(&format!("{}.v", module_name)) {
                matching_entries.push(entry.path().to_path_buf());
            }
        }

        if matching_entries.is_empty() {
            anyhow::bail!("No matching files found for module '{}'", module_name);
        } else if matching_entries.len() == 1 {
            process_file(&matching_entries[0], &target_path, extension, visited, package_name, &destination)?;
        } else {
            println!("Multiple modules found for '{}'. Please choose:", module_name);
            for (i, entry) in matching_entries.iter().enumerate() {
                println!("{}: {}", i + 1, entry.display());
            }

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice)?;
            let index: usize = choice.trim().parse()?;

            if index > 0 && index <= matching_entries.len() {
                process_file(&matching_entries[index - 1], &target_path, extension, visited, package_name, &destination)?;
            } else {
                anyhow::bail!("Invalid choice");
            }
    if let Some(entry) = find_module_file(&tmp_path, module, module_name) {
        process_file(&entry, &destination, &module_with_ext, url, visited, is_top_module)?;
    }

    let submodules = download_and_process_submodules(package_name, &module_with_ext, &destination, url, visited, is_top_module)?;

    Ok(submodules)
}

fn find_module_file(tmp_path: &Path, module: &str, module_name: &str) -> Option<DirEntry> {
    WalkDir::new(tmp_path)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| {
            let file_name = e.file_name().to_str().unwrap_or("");
            file_name == module || file_name == format!("{}.sv", module_name) || file_name == format!("{}.v", module_name)
        })
}

fn process_file(entry: &DirEntry, destination: &str, module_name: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool) -> Result<()> {
    let target_path = PathBuf::from(destination).join(module_name);
    let extension = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("v");

    fs::copy(entry.path(), &target_path)?;

    let contents = fs::read_to_string(entry.path())?;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())?;
    let tree = parser.parse(&contents, None).context("Failed to parse file")?;

    let header_content = generate_headers(tree.root_node(), &contents)?;
    let header_filename = format!("{}.{}", module_name.strip_suffix(".v").unwrap_or(module_name), if extension == "sv" { "svh" } else { "vh" });
    fs::write(PathBuf::from(destination).join(header_filename), header_content)?;

    update_lockfile(module_name, url, &contents, visited, is_top_module)?;

    Ok(())
}

fn download_and_process_submodules(package_name: &str, module_name: &str, destination: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool) -> Result<HashSet<String>> {
    let contents = fs::read_to_string(PathBuf::from(destination).join(module_name))?;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())?;
    let tree = parser.parse(&contents, None).context("Failed to parse file")?;

    let submodules = get_submodules(tree.root_node(), &contents)?;
    let mut all_submodules = HashSet::new();

    for submodule in submodules {
        let submodule_with_ext = if submodule.ends_with(".v") || submodule.ends_with(".sv") {
            submodule
        } else {
            format!("{}.v", &submodule)
        };
        if !visited.contains(&submodule_with_ext) {
            let submodule_destination = if is_top_module {
                PathBuf::from(destination).join("dependencies")
            } else {
                PathBuf::from(destination)
            };
            fs::create_dir_all(&submodule_destination)?;
            
            let submodule_url = format!("{}/{}", url, submodule_with_ext); // Assuming submodules are in subdirectories
            include_repo_from_url(&submodule_url, submodule_destination.to_str().unwrap())?;
            
            let processed_submodules = process_module(
                package_name,
                &submodule_with_ext,
                submodule_destination.to_str().unwrap().to_string(),
                visited,
                &submodule_url,
                false
            )?;
            
            all_submodules.insert(submodule_with_ext);
            all_submodules.extend(processed_submodules);
        }
    }

    Ok(all_submodules)
}

fn update_lockfile(module_name: &str, url: &str, contents: &str, visited: &HashSet<String>, is_top_module: bool) -> Result<()> {
    let mut lockfile = fs::read_to_string("vpm.lock").unwrap_or_default();
    let module_entry = if is_top_module {
        format!("[[package]]\nname = \"{}\"\nsource = \"{}\"\nparents = []", module_name, url)
    } else {
        format!("[[package]]\nname = \"{}\"\nsource = \"{}\"", module_name, url)
    };

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())?;
    let tree = parser.parse(contents, None).unwrap();
    let submodules = get_submodules(tree.root_node(), contents)?;
    let submodules_vec: Vec<String> = submodules.into_iter().map(|s| {
        if s.ends_with(".v") || s.ends_with(".sv") {
            s
        } else {
            format!("{}.v", s)
        }
    }).collect();

    if !lockfile.contains(&format!("name = \"{}\"", module_name)) {
        let formatted_submodules = submodules_vec.iter()
            .map(|s| format!("  \"{}\",", s))
            .collect::<Vec<_>>()
            .join("\n");
        lockfile.push_str(&format!("\n{}\nsubmodules = [\n{}\n]\n", module_entry, formatted_submodules));
    } else {
        update_submodules(&mut lockfile, &module_entry, &submodules_vec);
    }

    for submodule in &submodules_vec {
        if !visited.contains(submodule) {
            if let Some(existing_entry) = lockfile.find(&format!("\n[[package]]\nname = \"{}\"", submodule)) {
                let parent_start = lockfile[existing_entry..].find("parents = [").map(|i| existing_entry + i);
                if let Some(start) = parent_start {
                    let end = lockfile[start..].find(']').map(|i| start + i + 1).unwrap_or(lockfile.len());
                    let current_parents = lockfile[start..end].to_string();
                    let new_parents = if current_parents.contains(module_name) {
                        current_parents
                    } else {
                        format!("{}  \"{}\",\n]", &current_parents[..current_parents.len() - 1], module_name)
                    };
                    lockfile.replace_range(start..end, &new_parents);
                }
            } else {
                let submodule_entry = format!("\n[[package]]\nname = \"{}\"\nsource = \"{}\"\nparents = [\n  \"{}\",\n]\nsubmodules = []\n", submodule, url, module_name);
                lockfile.push_str(&submodule_entry);
            }
        }
    }

    fs::write("vpm.lock", lockfile)?;
    Ok(())
}

fn update_submodules(lockfile: &mut String, module_entry: &str, submodules: &[String]) {
    if let Some(start) = lockfile.find(module_entry).and_then(|pos| lockfile[pos..].find("submodules = [").map(|offset| pos + offset)) {
        let end = lockfile[start..].find(']').map(|pos| start + pos + 1).unwrap_or(lockfile.len());
        let new_modules = format!("submodules = [\n{}\n]", submodules.iter().map(|m| format!("  \"{}\",", m)).collect::<Vec<_>>().join("\n"));
        lockfile.replace_range(start..end, &new_modules);
    }
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