use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use fancy_regex::Regex;
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
        println!("Including from: '{}'", self.url);
        let repo_name = name_from_url(&self.url);
        let tmp_path = PathBuf::from("/tmp").join(repo_name);
        if self.repo {
            include_entire_repo(&self.url, &tmp_path)?
        } else {
            include_single_module(&self.url)?
        }
        Ok(())
    }
}

fn include_entire_repo(url: &str, tmp_path: &PathBuf) -> Result<()> {
    let url = format!("https://github.com/{}", url);
    println!("Full GitHub URL: {}", url);
    include_repo_from_url(&url, "/tmp/")?;
    add_dependency(&url, None)?;

    let files = get_files(&tmp_path.to_str().unwrap_or_default());
    let items = get_relative_paths(&files, tmp_path);

    let selected_items = select_modules(&items)?;

    process_selected_modules(&url, tmp_path, &selected_items)?;

    fs::remove_dir_all(tmp_path)?;
    print_success_message(&url, &selected_items);
    Ok(())
}

fn include_single_module(url: &str) -> Result<()> {
    let repo_url = get_github_repo_url(url).unwrap();
    include_repo_from_url(&repo_url, "/tmp/")?;
    add_dependency(&repo_url, None)?;
    println!("Repo URL: {}", repo_url);
    let module_path = get_component_path_from_github_url(url).unwrap_or_default();
    println!("Including module: {}", module_path);
    include_module_from_url(&module_path, &repo_url)?;
    let module_name = Path::new(&module_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&module_path);
    add_top_module(&repo_url, module_name)?;
    println!("Successfully installed module: {}", module_name);
    Ok(())
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

fn get_relative_paths(files: &[String], tmp_path: &PathBuf) -> Vec<String> {
    files.iter()
        .map(|file| file.strip_prefix(&tmp_path.to_string_lossy().as_ref())
            .unwrap_or(file)
            .trim_start_matches('/')
            .to_string())
        .collect()
}

fn select_modules(items: &[String]) -> Result<HashSet<String>> {
    let matcher = SkimMatcherV2::default();
    let mut selected_items: HashSet<String> = HashSet::new();

    loop {
        print!("Enter module name (or press Enter to finish): ");
        io::stdout().flush()?;

        let mut query = String::new();
        io::stdin().read_line(&mut query)?;
        query = query.trim().to_string();

        if query.is_empty() {
            break;
        }

        let filtered_items: Vec<String> = items
            .iter()
            .filter(|&item| matcher.fuzzy_match(item, &query).is_some())
            .cloned()
            .collect();

        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Toggle items to include with the space bar. Hit enter to start a new search")
            .items(&filtered_items)
            .interact()?;

        println!("\nSelected items:");
        for i in &selected_items {
            println!("- {}", i);
        }

        selected_items.extend(selection.iter().map(|&i| filtered_items[i].clone()));
    }

    print!("\x1B[2J\x1B[1;1H");
    Ok(selected_items)
}

fn process_selected_modules(url: &str, tmp_path: &PathBuf, selected_items: &HashSet<String>) -> Result<()> {
    for item in selected_items {
        let displayed_path = item.strip_prefix(tmp_path.to_string_lossy().as_ref()).unwrap_or(item).trim_start_matches('/');
        println!("Including module: {}", displayed_path);
        
        let full_path = tmp_path.join(displayed_path);
        let module_path = full_path.strip_prefix(tmp_path).unwrap_or(&full_path).to_str().unwrap().trim_start_matches('/');
        println!("Module path: {}", module_path);

        include_module_from_url(module_path, url)?;
        let module_name = Path::new(module_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(module_path);
        add_top_module(url, module_name)?;
    }

    if selected_items.is_empty() {
        println!("No modules selected. Including entire repository.");
        include_repo_from_url(url, "./vpm_modules/")?;
    }

    Ok(())
}

fn print_success_message(url: &str, selected_items: &HashSet<String>) {
    if !selected_items.is_empty() {
        let installed_modules = selected_items.iter()
            .filter_map(|item| {
                Path::new(item)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .collect::<Vec<String>>()
            .join(", ");
        println!("Successfully installed module(s): {}", installed_modules);
    } else {
        println!("Successfully installed repository '{}'.", name_from_url(url));
    }
}

fn name_from_url(url: &str) -> &str {
    url.rsplit('/').find(|&s| !s.is_empty()).unwrap_or_default()
}

fn get_component_path_from_github_url(url: &str) -> Option<String> {
    let parts: Vec<&str> = url.split("/").collect();
    if parts.len() < 8 || !url.starts_with("https://github.com/") {
        return None;
    }

    Some(parts[7..].join("/"))
}

fn get_github_repo_url(url: &str) -> Option<String> {
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() < 5 || !url.starts_with("https://github.com/") {
        return None;
    }

    Some(format!("https://github.com/{}/{}", parts[3], parts[4]))
}

fn is_full_filepath(path: &str) -> bool {
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

    println!("Including submodule '{}'", module_with_ext);

    let mut processed_modules = HashSet::new();

    if is_full_filepath(&module_with_ext) {
        let dir_entry = filepath_to_dir_entry(file_path)?;
        process_file(&dir_entry, &target_path.to_str().unwrap(), module_name, url, visited, is_top_module)?;
        processed_modules.insert(module_with_ext.clone());
    } else {
        process_non_full_filepath(module_name, &tmp_path, &target_path, url, visited, is_top_module, &mut processed_modules)?;
    }

    let submodules = download_and_process_submodules(package_name, &module_with_ext, &destination, url, visited, is_top_module)?;
    processed_modules.extend(submodules);

    Ok(processed_modules)
}

fn process_non_full_filepath(module_name: &str, tmp_path: &PathBuf, target_path: &PathBuf, url: &str, visited: &mut HashSet<String>, is_top_module: bool, processed_modules: &mut HashSet<String>) -> Result<()> {
    let matching_entries = find_matching_entries(module_name, tmp_path);

    if matching_entries.is_empty() {
        println!("No matching files found for module '{}'. Skipping...", module_name);
        return Ok(());
    } else if matching_entries.len() == 1 {
        let dir_entry = filepath_to_dir_entry(matching_entries[0].clone())?;
        process_file(&dir_entry, target_path.to_str().unwrap(), module_name, url, visited, is_top_module)?;
        processed_modules.insert(format!("{}.v", module_name));
    } else {
        process_multiple_matches(matching_entries, target_path, module_name, url, visited, is_top_module, processed_modules)?;
    }

    Ok(())
}

fn find_matching_entries(module_name: &str, tmp_path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(tmp_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_name().to_str() == Some(&format!("{}.sv", module_name)) || 
            entry.file_name().to_str() == Some(&format!("{}.v", module_name))
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

fn process_multiple_matches(matching_entries: Vec<PathBuf>, target_path: &PathBuf, module_name: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool, processed_modules: &mut HashSet<String>) -> Result<()> {
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
        processed_modules.insert(format!("{}.v", module_name));
    } else {
        anyhow::bail!("Invalid choice");
    }

    Ok(())
}

fn process_file(entry: &DirEntry, destination: &str, module_path: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool) -> Result<()> {
    let target_path = PathBuf::from(destination);
    let extension = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("v");
    fs::copy(entry.path(), &target_path.join(entry.file_name()))?;

    let contents = fs::read_to_string(entry.path())?;
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
    // let mut parser = Parser::new();
    // parser.set_language(tree_sitter_verilog::language())?;
    // let tree = parser.parse(&contents, None).context("Failed to parse file")?;

    let submodules = get_submodules(&contents)?;
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

    // let mut parser = Parser::new();
    // parser.set_language(tree_sitter_verilog::language())?;
    // let tree = parser.parse(contents, None).unwrap();
    let submodules = get_submodules(contents)?;
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

pub fn get_submodules(contents: &str) -> Result<HashSet<String>> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
        r"(?mi)^\s*(?!(always(_comb|_ff|_latch)?|assert|assign|assume|begin|case|cover|else|end(case|function|generate|module|primitive|table|task)?|enum|for|forever|function|generate|if|initial|input|int|localparam|logic|module|negedge|output|param(eter)?|posedge|primitive|real|reg|repeat|table|task|time|timescale|typedef|while|wire))(\w+)\s*(?:#\([\s.\w(\[\-:\]\),{'}`/+!~@#$%^&*=<>?]+\))?\s*[\w\[:\]]+\s*(?:\([\s.\w(\[\-:\]\),{'}`/+!~@#$%^&*=<>?]+\));"
    ).unwrap());
    let submodules: HashSet<String> = REGEX
        .captures_iter(contents) // Iterate over captures
        .map(|caps| caps.unwrap().get(0).unwrap().as_str()) // Extract the matched string
        .map(|s| s.split_whitespace().next().unwrap().to_string()) // Split and get submodule name
        .collect(); // Collect into a HashSet
    for submodule in &submodules {
        println!("Found submodule: {}", submodule);
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