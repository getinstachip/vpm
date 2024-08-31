use std::collections::HashSet;

use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use crate::cmd::{Execute, Include};
use crate::toml::{add_dependency, add_top_module};
use walkdir::{DirEntry, WalkDir};

use dialoguer::{theme::ColorfulTheme, MultiSelect};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::io::{self, Write};


impl Execute for Include {
    async fn execute(&self) -> Result<()> {
        fs::create_dir_all("./vpm_modules")?;
        println!("Including repository from URL: '{}'", self.url);
        let repo_name = name_from_url(&self.url);
        let tmp_path = PathBuf::from("/tmp").join(repo_name);
        include_repo_from_url(&self.url, "/tmp/")?;
        add_dependency(&self.url, None)?;

        let files = get_files(&tmp_path.to_str().unwrap_or_default());

        let items: Vec<String> = files
            .into_iter()
            .map(|file| file.strip_prefix(&tmp_path.to_string_lossy().as_ref())
                .unwrap_or(&file)
                .trim_start_matches('/')
                .to_string())
            .collect();


        let matcher = SkimMatcherV2::default();
        let mut _filtered_items = items.clone();
        let mut query = String::new();

        let mut selected_items: HashSet<String> = HashSet::new();

        loop {
            print!("Enter module name (or press Enter to finish): ");
            io::stdout().flush()?;

            query.clear();
            io::stdin().read_line(&mut query)?;
            query = query.trim().to_string();

            if query.is_empty() {
                break;
            }

            _filtered_items = items
                .iter()
                .filter(|&item| matcher.fuzzy_match(item, &query).is_some())
                .cloned()
                .collect();

            let selection = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Toggle items to include with the space bar. Hit enter to start a new search")
                .items(&_filtered_items)
                .interact()?;

            println!("\nSelected items:");
            for i in &selected_items {
                println!("- {}", i);
            }
    
            selected_items.extend(selection.iter().map(|&i| _filtered_items[i].clone()));
        }

        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        let has_selected_items = !selected_items.is_empty();

        for item in &selected_items {
            let item_text = item.clone();
            let displayed_path = item_text.strip_prefix(tmp_path.to_string_lossy().as_ref()).unwrap_or(&item_text).trim_start_matches('/');
            println!("Including module: {}", displayed_path);
            
            let full_path = tmp_path.join(displayed_path);
            let module_path = full_path.strip_prefix(&tmp_path).unwrap_or(&full_path).to_str().unwrap().trim_start_matches('/');
            
            include_module_from_url(module_path, &self.url)?;
            let module_name = Path::new(module_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(module_path);
            add_top_module(&self.url, module_name)?;
        }

        if !has_selected_items {
            println!("No modules selected. Including entire repository.");
            include_repo_from_url(&self.url, "./vpm_modules/")?;
        }

        fs::remove_dir_all(tmp_path)?;
        if has_selected_items {
            let installed_modules = selected_items.iter()
                .filter_map(|item| {
                    let item_text = item.clone();
                    Path::new(&item_text)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .collect::<Vec<String>>()
                .join(", ");
            println!("Successfully installed module(s): {}", installed_modules);
        } else {
            println!("Successfully installed repository '{}'.", name_from_url(&self.url));
        }
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

fn filepath_to_dir_entry(filepath: PathBuf) -> Result<DirEntry> {
    WalkDir::new(filepath)
        .min_depth(0)
        .max_depth(0)
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to create DirEntry"))?
        .context("Failed to create DirEntry")
}


pub fn include_module_from_url(module_path: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url);

    include_repo_from_url(url, "/tmp/")?;
    let module_name = Path::new(module_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(module_path);
    let destination = format!("./vpm_modules/{}", module_name);
    fs::create_dir_all(&destination)?;
    // fs::create_dir_all(format!("{}/dependencies", &destination))?;
    process_module(package_name, module_path, destination.to_owned(), &mut HashSet::new(), url, true)?;
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
    let file_path = tmp_path.join(&module_with_ext);

    let target_path = PathBuf::from(&destination);
    // println!("Target path: {}", target_path.display());

    println!("Including submodule '{}'", module_with_ext);

    let mut processed_modules = HashSet::new();

    if is_full_filepath(&module_with_ext) {
        let dir_entry = filepath_to_dir_entry(file_path)?;
        process_file(&dir_entry, &target_path.to_str().unwrap(), module_name, url, visited, is_top_module)?;
        processed_modules.insert(module_with_ext.clone());
        // println!("Processed file: {}", dir_entry.path().to_str().unwrap());
    } else {
        let mut matching_entries = Vec::new();
        for entry in WalkDir::new(&tmp_path).into_iter().filter_map(Result::ok) {
            if entry.file_name().to_str() == Some(&format!("{}.sv", module_name)) || entry.file_name().to_str() == Some(&format!("{}.v", module_name)) {
                matching_entries.push(entry.path().to_path_buf());
                // println!("Matching entry: {}", entry.path().to_str().unwrap());
            }
        }

        if matching_entries.is_empty() {
            anyhow::bail!("No matching files found for module '{}'", module_name);
        } else if matching_entries.len() == 1 {
            let dir_entry = filepath_to_dir_entry(matching_entries[0].clone())?;
            // println!("Processing file: {}", dir_entry.path().to_str().unwrap());
            process_file(&dir_entry, target_path.to_str().unwrap(), module_name, url, visited, is_top_module)?;
            // println!("Processed file: {}", dir_entry.path().to_str().unwrap());
            processed_modules.insert(module_with_ext.clone());
        } else {
            println!("Multiple modules found for '{}'. Please choose:", module_name);
            for (i, entry) in matching_entries.iter().enumerate() {
                println!("{}: {}", i + 1, entry.display());
            }

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice)?;
            let index: usize = choice.trim().parse()?;

            if index > 0 && index <= matching_entries.len() {
                let dir_entry = filepath_to_dir_entry(matching_entries[index - 1].clone())?;
                process_file(&dir_entry, target_path.to_str().unwrap(), module_name, url, visited, is_top_module)?;
                processed_modules.insert(module_with_ext.clone());
            } else {
                anyhow::bail!("Invalid choice");
            }
        }
    }
    // println!("Destination: {}", destination);
    let submodules = download_and_process_submodules(package_name, &module_with_ext, &destination, url, visited, is_top_module)?;
    processed_modules.extend(submodules);

    Ok(processed_modules)
}

fn process_file(entry: &DirEntry, destination: &str, module_path: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool) -> Result<()> {
    let target_path = PathBuf::from(destination);
    let extension = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("v");
    // println!("Copying file: {}", entry.path().to_str().unwrap());
    // println!("Target path: {}", target_path.display());
    fs::copy(entry.path(), &target_path.join(entry.file_name()))?;
    // println!("Copied file: {}", entry.path().to_str().unwrap());

    let contents = fs::read_to_string(entry.path())?;
    // println!("Read file: {}", entry.path().to_str().unwrap());
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())?;
    let tree = parser.parse(&contents, None).context("Failed to parse file")?;

    let header_content = generate_headers(tree.root_node(), &contents)?;
    let module_name = Path::new(module_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(module_path);
    let module_name_with_ext = if !module_name.ends_with(".v") && !module_name.ends_with(".sv") {
        format!("{}.{}", module_name, extension)
    } else {
        module_name.to_string()
    };
    let header_filename = format!("{}.{}", module_name.strip_suffix(".v").unwrap_or(module_name), if extension == "sv" { "svh" } else { "vh" });
    // println!("Writing header file: {}", target_path.join(&header_filename).to_str().unwrap());
    fs::write(target_path.join(&header_filename), header_content)?;
    println!("Generating header file: {}", target_path.join(&header_filename).to_str().unwrap());

    update_lockfile(module_name_with_ext.as_str(), url, &contents, visited, is_top_module)?;

    Ok(())
}

fn download_and_process_submodules(package_name: &str, module_path: &str, destination: &str, url: &str, visited: &mut HashSet<String>, _is_top_module: bool) -> Result<HashSet<String>> {
    let module_name = Path::new(module_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(module_path);

    let module_name_with_ext = if module_path.ends_with(".sv") {
        format!("{}.sv", module_name)
    } else if module_path.ends_with(".v") {
        format!("{}.v", module_name)
    } else {
        module_path.to_string()
    };

    let contents = fs::read_to_string(PathBuf::from(destination).join(module_name_with_ext))?;
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
            let submodule_destination = PathBuf::from(destination);
            fs::create_dir_all(&submodule_destination)?;
            
            let submodule_url = format!("{}/{}", url, submodule_with_ext);
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
            "`ifndef {}_H\n`define {}_H\n\n",
            module_name.to_uppercase(),
            module_name.to_uppercase()
        ));

        if !params.is_empty() {
            header_content.push_str(&format!(
                "// Parameters\n{}\n\n",
                params.trim()
            ));
        }

        if !ports.is_empty() {
            header_content.push_str(&format!(
                "// Ports\n{}\n\n",
                ports.trim()
            ));
        }

        header_content.push_str(&format!(
            "// Module: {}\n// TODO: Add module description\n\n`endif // {}_H\n\n",
            module_name,
            module_name.to_uppercase()
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