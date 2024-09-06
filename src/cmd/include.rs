use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use crate::cmd::{Execute, Include};
use crate::toml::{add_dependency, add_top_module};
use walkdir::{DirEntry, WalkDir};
use fancy_regex::Regex;

use dialoguer::{theme::ColorfulTheme, MultiSelect};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::io::{self, Write};

impl Execute for Include {
    async fn execute(&self) -> Result<()> {
        fs::create_dir_all("./vpm_modules").context("Failed to create vpm_modules directory. Please check if you have write permissions in the current directory.")?;
        println!("Including from: '{}'", self.url);
        let repo_name = name_from_url(&self.url);
        let tmp_path = PathBuf::from("/tmp").join(repo_name);
        if self.repo {
            include_entire_repo(&self.url, &tmp_path, self.riscv).context("Failed to include entire repository. Please check your internet connection and the repository URL.")?
        } else {
            include_single_module(&self.url, self.riscv).context("Failed to include single module. Please verify the module URL and ensure it exists in the repository.")?
        }
        Ok(())
    }
}

fn include_entire_repo(url: &str, tmp_path: &PathBuf, riscv: bool) -> Result<()> {
    let url = format!("https://github.com/{}", url);
    println!("Full GitHub URL: {}", url);
    include_repo_from_url(&url, "/tmp/").context("Failed to access repository. Please check your internet connection and the repository URL.")?;
    add_dependency(&url, None).context("Failed to add dependency to vpm.toml. Please ensure you have write permissions for this file.")?;

    let files = get_files(&tmp_path.to_str().unwrap_or_default());
    let items = get_relative_paths(&files, tmp_path);

    let selected_items = select_modules(&items).context("Failed to select modules. Please try again and ensure you make a valid selection.")?;

    process_selected_modules(&url, tmp_path, &selected_items, riscv).context("Failed to process selected modules. Please check the module files for any syntax errors.")?;

    fs::remove_dir_all(tmp_path).context("Failed to remove temporary directory. You may need to manually delete it.")?;
    print_success_message(&url, &selected_items);
    Ok(())
}

fn include_single_module(url: &str, riscv: bool) -> Result<()> {
    let repo_url = get_github_repo_url(url).ok_or_else(|| anyhow::anyhow!("Invalid GitHub URL. Please provide a valid GitHub repository URL."))?;
    include_repo_from_url(&repo_url, "/tmp/").context("Failed to access repository. Please check your internet connection and the repository URL.")?;
    add_dependency(&repo_url, None).context("Failed to add dependency to vpm.toml. Please ensure you have write permissions for this file.")?;
    println!("Repo URL: {}", repo_url);
    let module_path = get_component_path_from_github_url(url).ok_or_else(|| anyhow::anyhow!("Failed to extract module path from URL. Please check the URL format."))?;
    println!("Including module: {}", module_path);
    include_module_from_url(&module_path, &repo_url, riscv).context("Failed to include module. Please check if the module exists in the repository and has no syntax errors.")?;
    println!("Successfully installed module: {}", module_path);
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
        io::stdout().flush().context("Failed to flush stdout. This may be a system-level issue.")?;

        let mut query = String::new();
        io::stdin().read_line(&mut query).context("Failed to read user input. Please try again.")?;
        query = query.trim().to_string();

        if query.is_empty() {
            break;
        }

        let filtered_items: Vec<String> = items
            .iter()
            .filter(|&item| matcher.fuzzy_match(item, &query).is_some())
            .cloned()
            .collect();

        if filtered_items.is_empty() {
            println!("No matching modules found. Please try a different search term.");
            continue;
        }

        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Toggle items to include with the space bar. Hit enter to start a new search")
            .items(&filtered_items)
            .interact()
            .context("Failed to get user selection. Please try again.")?;

        for i in &selected_items {
            println!("- {}", i);
        }

        selected_items.extend(selection.iter().map(|&i| filtered_items[i].clone()));
    }

    print!("\x1B[2J\x1B[1;1H");
    Ok(selected_items)
}

fn process_selected_modules(url: &str, tmp_path: &PathBuf, selected_items: &HashSet<String>, riscv: bool) -> Result<()> {
    for item in selected_items {
        let displayed_path = item.strip_prefix(tmp_path.to_string_lossy().as_ref()).unwrap_or(item).trim_start_matches('/');
        println!("Including module: {}", displayed_path);
        
        let full_path = tmp_path.join(displayed_path);
        let module_path = full_path.strip_prefix(tmp_path).unwrap_or(&full_path).to_str().unwrap().trim_start_matches('/');
        println!("Module path: {}", module_path);

        include_module_from_url(module_path, url, riscv).context(format!("Failed to include module: {}. Please check if the module file exists and has no syntax errors.", module_path))?;
    }

    if selected_items.is_empty() {
        println!("No modules selected. Including entire repository.");
        include_repo_from_url(url, "./vpm_modules/").context("Failed to include entire repository. Please check your internet connection and the repository URL.")?;
    }

    Ok(())
}

fn print_success_message(url: &str, selected_items: &HashSet<String>) {
    if !selected_items.is_empty() {
        let installed_modules = selected_items.iter()
            .map(|item| item.to_string())
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
        .ok_or_else(|| anyhow::anyhow!("Failed to create DirEntry. The file path may be invalid or inaccessible."))
        .and_then(|entry| entry.context("Failed to create DirEntry. The file may not exist or you may not have permission to access it."))
}

fn generate_top_v_content(module_path: &str) -> Result<String> {
    println!("Generating top.v file for RISC-V in {}", module_path);
    let module_content = fs::read_to_string(module_path).context("Failed to read module file. Please check if the file exists and you have read permissions.")?;

    let mut top_content = String::new();
    top_content.push_str("// Auto-generated top.v file for RISC-V\n\n");

    // Use regex to find module declaration
    let module_re = regex::Regex::new(r"module\s+(\w+)\s*(?:#\s*\(([\s\S]*?)\))?\s*\(([\s\S]*?)\);").context("Failed to create regex for module declaration. This is likely a bug in VPM.")?;
    if let Some(captures) = module_re.captures(&module_content) {
        let module_name = captures.get(1).unwrap().as_str();
        println!("Module name: {}", module_name);

        // Extract parameters
        let params = captures.get(2).map_or(Vec::new(), |m| {
            m.as_str().lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect()
        });

        // Extract ports
        let ports: Vec<&str> = captures.get(3).unwrap().as_str()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        // Generate top module ports
        top_content.push_str("module top (\n");
        for port in &ports {
            top_content.push_str(&format!("    {}\n", port));
        }
        top_content.push_str(");\n\n");

        // Instantiate the module
        top_content.push_str(&format!("{} #(\n", module_name));
        for param in params.iter() {
            if let Some((name, value)) = param.split_once('=') {
                let name = name.trim().trim_start_matches("parameter").trim();
                let name = name.split_whitespace().last().unwrap_or(name);
                let value = value.trim().trim_end_matches(',');
                top_content.push_str(&format!("    .{}({}),\n", name, value));
            }
        }
        top_content.push_str(") cpu (\n");

        // Connect ports
        let port_re = regex::Regex::new(r"(input|output|inout)\s+(?:wire|reg)?\s*(?:\[.*?\])?\s*(\w+)").context("Failed to create regex for port declaration. This is likely a bug in VPM.")?;
        for (i, port) in ports.iter().enumerate() {
            if let Some(port_captures) = port_re.captures(port) {
                let port_name = port_captures.get(2).unwrap().as_str();
                top_content.push_str(&format!("    .{}({}){}\n", port_name, port_name, if i < ports.len() - 1 { "," } else { "" }));
            }
        }
        top_content.push_str(");\n\n");

        top_content.push_str("endmodule\n");
        return Ok(top_content);
    }

    Err(anyhow::anyhow!("No module declaration found in the file. Please check if the Verilog file is correctly formatted."))
}

fn generate_xdc_content(module_path: &str) -> Result<String> {
    println!("Generating constraints.xdc file for Xilinx Artix-7 board in {}", module_path);
    let module_content = fs::read_to_string(module_path).context("Failed to read module file. Please check if the file exists and you have read permissions.")?;

    let mut xdc_content = String::new();
    xdc_content.push_str("## Auto-generated constraints.xdc file for Xilinx Artix-7 board\n\n");

    // Use regex to find all ports
    let port_re = regex::Regex::new(r"(?m)^\s*(input|output|inout)\s+(?:wire|reg)?\s*(?:\[.*?\])?\s*(\w+)").context("Failed to create regex for port declaration. This is likely a bug in VPM.")?;
    let mut ports = Vec::new();

    for captures in port_re.captures_iter(&module_content) {
        let port_type = captures.get(1).unwrap().as_str();
        let port_name = captures.get(2).unwrap().as_str();
        ports.push((port_type, port_name));
    }

    // Define pin mappings (you may need to adjust these based on your specific board)
    let pin_mappings = [
        ("clk", "E3"),
        ("resetn", "C12"),
        ("trap", "D10"),
        ("mem_valid", "C11"),
        ("mem_instr", "C10"),
        ("mem_ready", "A10"),
        ("mem_addr[0]", "A8"),
        ("mem_wdata[0]", "C5"),
        ("mem_wstrb[0]", "C6"),
        ("mem_rdata[0]", "D5"),
    ];

    // Generate constraints for each port
    for (port_type, port_name) in ports {
        if let Some((_, pin)) = pin_mappings.iter().find(|&&(p, _)| p == port_name) {
            let iostandard = if port_name == "clk" { "LVCMOS33" } else { "LVCMOS33" };
            xdc_content.push_str(&format!("set_property -dict {{ PACKAGE_PIN {} IOSTANDARD {} }} [get_ports {{ {} }}]\n", pin, iostandard, port_name));
        } else {
            println!("Warning: No pin mapping found for port: {}. You may need to manually add this to the XDC file.", port_name);
        }
    }

    // Add clock constraint
    if let Some((_, clk_pin)) = pin_mappings.iter().find(|&&(p, _)| p == "clk") {
        xdc_content.push_str(&format!("\n## Clock signal\n"));
        xdc_content.push_str(&format!("create_clock -period 10.000 -name sys_clk_pin -waveform {{0.000 5.000}} -add [get_ports {{ clk }}]\n"));
    } else {
        println!("Warning: No clock signal found. XDC file may be incomplete.");
        xdc_content.push_str("\n## Warning: No clock signal found. Please add clock constraints manually.\n");
    }

    Ok(xdc_content)
}

pub fn include_module_from_url(module_path: &str, url: &str, riscv: bool) -> Result<()> {
    let package_name = name_from_url(url);

    include_repo_from_url(url, "/tmp/").context("Failed to include repository from URL. Please check your internet connection and ensure the URL is correct.")?;
    let module_name = Path::new(module_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(module_path);
    let destination = format!("./vpm_modules/{}", module_name);
    fs::create_dir_all(&destination).context("Failed to create destination directory. Please check if you have write permissions in the current directory.")?;
    process_module(package_name, module_path, destination.to_owned(), &mut HashSet::new(), url, true)
        .context("Failed to process module. This could be due to invalid module content or file structure.")?;
    
    let module_file_name = Path::new(&destination)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid destination path. Please ensure the path is correctly formatted and accessible."))?;
    let module_path = Path::new(&destination).join(format!("{}.v", module_file_name));

    if !module_path.exists() {
        let module_path = Path::new(&destination).join(format!("{}.sv", module_file_name));
        if !module_path.exists() {
            return Err(anyhow::anyhow!("Module file not found in the destination folder. Please check if the module was correctly downloaded and processed."));
        }
    }

    if riscv {
        let top_v_content = generate_top_v_content(&module_path.to_str().unwrap())
            .context("Failed to generate top.v content. This could be due to issues with the module file or incorrect RISC-V configuration.")?;
        fs::write(format!("{}/top.v", destination), top_v_content)
            .context("Failed to write top.v file. Please check if you have write permissions in the destination directory.")?;
        println!("Created top.v file for RISC-V in {}", destination);
        // Generate .xdc file for Xilinx Artix-7 board
        let xdc_content = generate_xdc_content(&format!("{}/top.v", destination))
            .context("Failed to generate XDC content. This could be due to issues with the top.v file or incorrect board configuration.")?;
        fs::write(format!("{}/constraints.xdc", destination), xdc_content)
            .context("Failed to write constraints.xdc file. Please check if you have write permissions in the destination directory.")?;
        println!("Created constraints.xdc file for Xilinx Artix-7 board in {}", destination);
    }
    add_top_module(url, module_path.to_str().unwrap())
        .context("Failed to add top module. This could be due to issues with the module file or incorrect URL.")?;
    
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
        println!("Full filepath detected for module '{}'", module_with_ext);
        let dir_entry = filepath_to_dir_entry(file_path)
            .context("Failed to convert filepath to directory entry. Please ensure the file exists and is accessible.")?;
        process_file(&dir_entry, &target_path.to_str().unwrap(), module, url, visited, is_top_module)
            .context("Failed to process file. This could be due to file access issues or invalid file content.")?;
        processed_modules.insert(module_with_ext.clone());
    } else {
        println!("Full filepath not detected for module '{}'", module_with_ext);
        process_non_full_filepath(module_name, &tmp_path, &target_path, url, visited, is_top_module, &mut processed_modules)
            .context("Failed to process non-full filepath. This could be due to missing files or incorrect module structure.")?;
    }

    let submodules = download_and_process_submodules(package_name, module, &destination, url, visited, is_top_module)
        .context("Failed to download and process submodules. This could be due to network issues or invalid submodule references.")?;
    processed_modules.extend(submodules);

    Ok(processed_modules)
}

fn process_non_full_filepath(module_name: &str, tmp_path: &PathBuf, target_path: &PathBuf, url: &str, visited: &mut HashSet<String>, is_top_module: bool, processed_modules: &mut HashSet<String>) -> Result<()> {
    let matching_entries = find_matching_entries(module_name, tmp_path);
    println!("Found {} matching entries for module '{}'", matching_entries.len(), module_name);
    if matching_entries.is_empty() {
        println!("No matching files found for module '{}'. Skipping... This could mean the module file is missing or named incorrectly.", module_name);
    } else if matching_entries.len() == 1 {
        let dir_entry = filepath_to_dir_entry(matching_entries[0].clone())
            .context("Failed to convert filepath to directory entry. Please ensure the file exists and is accessible.")?;
        process_file(&dir_entry, target_path.to_str().unwrap(), module_name, url, visited, is_top_module)
            .context("Failed to process file. This could be due to file access issues or invalid file content.")?;
        processed_modules.insert(format!("{}.v", module_name));
    } else {
        process_multiple_matches(matching_entries, target_path, module_name, url, visited, is_top_module, processed_modules)
            .context("Failed to process multiple matching files. This could be due to user input errors or file access issues.")?;
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
    std::io::stdin().read_line(&mut choice)
        .context("Failed to read user input. Please try again.")?;
    let index: usize = choice.trim().parse()
        .context("Invalid input. Please enter a number corresponding to one of the listed options.")?;

    if index > 0 && index <= matching_entries.len() {
        let dir_entry = filepath_to_dir_entry(matching_entries[index - 1].clone())
            .context("Failed to convert filepath to directory entry. Please ensure the file exists and is accessible.")?;
        process_file(&dir_entry, target_path.to_str().unwrap(), module_name, url, visited, is_top_module)
            .context("Failed to process file. This could be due to file access issues or invalid file content.")?;
        processed_modules.insert(format!("{}.v", module_name));
    } else {
        anyhow::bail!("Invalid choice. Please enter a number between 1 and {}.", matching_entries.len());
    }

    Ok(())
}

fn process_file(entry: &DirEntry, destination: &str, module_path: &str, url: &str, visited: &mut HashSet<String>, is_top_module: bool) -> Result<()> {
    let target_path = PathBuf::from(destination);
    let extension = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("v");
    fs::copy(entry.path(), &target_path.join(entry.file_name()))
        .context("Failed to copy file. Please check if you have read permissions for the source file and write permissions for the destination.")?;

    let contents = fs::read_to_string(entry.path())
        .context("Failed to read file contents. Please check if the file exists and you have read permissions.")?;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_verilog::language())
        .context("Failed to set parser language. This is likely a bug in VPM.")?;
    let tree = parser.parse(&contents, None)
        .context("Failed to parse file. The file may contain syntax errors or be in an unsupported format.")?;

    let header_content = generate_headers(tree.root_node(), &contents)
        .context("Failed to generate headers. This could be due to unexpected file structure or content.")?;
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
    fs::write(target_path.join(&header_filename), header_content)
        .context("Failed to write header file. Please check if you have write permissions in the target directory.")?;
    println!("Generating header file: {}", target_path.join(&header_filename).to_str().unwrap());

    let full_module_path = target_path.join(&module_name_with_ext);
    update_lockfile(&full_module_path, url, &contents, visited, is_top_module)
        .context("Failed to update lockfile. This could be due to file access issues or concurrent modifications.")?;

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

    let full_module_path = PathBuf::from(destination).join(&module_name_with_ext);
    let contents = match fs::read_to_string(&full_module_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Warning: Failed to read file {}: {}. Skipping this module. Please check if the file exists and you have read permissions.", full_module_path.display(), e);
            return Ok(HashSet::new());
        }
    };
    
    let mut parser = Parser::new();
    if let Err(e) = parser.set_language(tree_sitter_verilog::language()) {
        eprintln!("Warning: Failed to set parser language: {}. Skipping submodule processing. This is likely a bug in VPM.", e);
        return Ok(HashSet::new());
    }

    let submodules = match get_submodules(&contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Warning: Failed to get submodules from {}: {}. Continuing without submodules. This could be due to unexpected file content or structure.", full_module_path.display(), e);
            HashSet::new()
        }
    };

    let mut all_submodules = HashSet::new();

    for submodule in submodules {
        let submodule_with_ext = if submodule.ends_with(".v") || submodule.ends_with(".sv") {
            submodule
        } else {
            format!("{}.v", &submodule)
        };
        if !visited.contains(&submodule_with_ext) {
            let submodule_destination = PathBuf::from(destination);
            if let Err(e) = fs::create_dir_all(&submodule_destination) {
                eprintln!("Warning: Failed to create directory {}: {}. Skipping this submodule. Please check if you have write permissions in the parent directory.", submodule_destination.display(), e);
                continue;
            }
            
            let submodule_url = format!("{}/{}", url, submodule_with_ext);
            if let Err(e) = include_repo_from_url(&submodule_url, submodule_destination.to_str().unwrap()) {
                eprintln!("Warning: Failed to include repo from URL {}: {}. Skipping this submodule. Please check your internet connection and ensure the URL is correct.", submodule_url, e);
                continue;
            }
            
            match process_module(
                package_name,
                &submodule_with_ext,
                submodule_destination.to_str().unwrap().to_string(),
                visited,
                &submodule_url,
                false
            ) {
                Ok(processed_submodules) => {
                    all_submodules.insert(submodule_with_ext.clone());
                    all_submodules.extend(processed_submodules);
                },
                Err(e) => {
                    eprintln!("Warning: Failed to process submodule {}: {}. Skipping this submodule. This could be due to issues with the submodule file or structure.", submodule_with_ext, e);
                    continue;
                }
            }

            let full_submodule_path = submodule_destination.join(&submodule_with_ext);
            if let Err(e) = update_lockfile(&full_submodule_path, &submodule_url, &contents, visited, false) {
                eprintln!("Warning: Failed to update lockfile for {}: {}. Continuing without updating lockfile.", full_submodule_path.display(), e);
            }
        }
    }

    Ok(all_submodules)
}

fn update_lockfile(full_path: &PathBuf, url: &str, contents: &str, visited: &HashSet<String>, is_top_module: bool) -> Result<()> {
    let mut lockfile = fs::read_to_string("vpm.lock").unwrap_or_default();
    let full_module_path = format!("{}/{}", url, full_path.file_name().unwrap().to_str().unwrap());
    let module_entry = if is_top_module {
        format!("[[package]]\nfull_path = \"{}\"\nsource = \"{}\"\nparents = []\n", full_path.display(), url)
    } else {
        format!("[[package]]\nfull_path = \"{}\"\nsource = \"{}\"\n", full_path.display(), url)
    };

    let mut parser = Parser::new();
    if let Err(e) = parser.set_language(tree_sitter_verilog::language()) {
        return Err(anyhow::anyhow!("Failed to set Verilog language for parser: {}. This might be due to an incompatible tree-sitter-verilog version. Try updating your dependencies.", e));
    }

    let submodules = match get_submodules(contents) {
        Ok(s) => s,
        Err(e) => return Err(anyhow::anyhow!("Failed to get submodules: {}. This could be due to malformed Verilog code. Please check your module for syntax errors.", e)),
    };
    let submodules_vec: Vec<String> = submodules.into_iter().collect();

    if !lockfile.contains(&format!("full_path = \"{}\"", full_path.display())) {
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
            let submodule_path = full_path.parent().unwrap().join(submodule);
            if let Some(existing_entry) = lockfile.find(&format!("\n[[package]]\nfull_path = \"{}\"", submodule_path.display())) {
                let parent_start = lockfile[existing_entry..].find("parents = [").map(|i| existing_entry + i);
                if let Some(start) = parent_start {
                    let end = lockfile[start..].find(']').map(|i| start + i + 1).unwrap_or(lockfile.len());
                    let current_parents = lockfile[start..end].to_string();
                    let new_parents = if current_parents.contains(&full_path.display().to_string()) {
                        current_parents
                    } else {
                        format!("{}  \"{}\",\n]", &current_parents[..current_parents.len() - 1], full_path.display())
                    };
                    lockfile.replace_range(start..end, &new_parents);
                }
            } else {
                let submodule_entry = format!("\n[[package]]\nfull_path = \"{}\"\nsource = \"{}\"\nparents = [\n  \"{}\",\n]\nsubmodules = []\n", submodule_path.display(), url, full_path.display());
                lockfile.push_str(&submodule_entry);
            }
        }
    }

    if let Err(e) = fs::write("vpm.lock", &lockfile) {
        return Err(anyhow::anyhow!("Failed to write to vpm.lock: {}. This could be due to insufficient permissions or disk space. Please check your file system and try again.", e));
    }
    Ok(())
}

fn update_submodules(lockfile: &mut String, module_entry: &str, submodules: &[String]) {
    if let Some(start) = lockfile.find(module_entry).and_then(|pos| lockfile[pos..].find("submodules = [").map(|offset| pos + offset)) {
        let end = lockfile[start..].find(']').map(|pos| start + pos + 1).unwrap_or(lockfile.len());
        let new_modules = format!("submodules = [\n{}\n]", submodules.iter().map(|m| format!("  \"{}\",", m)).collect::<Vec<_>>().join("\n"));
        lockfile.replace_range(start..end, &new_modules);
    } else {
        eprintln!("Warning: Could not find submodules entry for module {}. The lockfile may be corrupted or in an unexpected format.", module_entry);
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

    if header_content.is_empty() {
        return Err(anyhow::anyhow!("No module declarations found in the content. Please ensure your Verilog file contains valid module declarations."));
    }

    Ok(header_content)
}


pub fn get_submodules(contents: &str) -> Result<HashSet<String>> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
        r"(?mi)^\s*(?!(always(_comb|_ff|_latch)?|assert|assign|assume|begin|case|cover|else|end(case|function|generate|module|primitive|table|task)?|enum|for|forever|function|generate|if|initial|input|int|localparam|logic|module|negedge|output|param(eter)?|posedge|primitive|real|reg|repeat|table|task|time|timescale|typedef|while|wire))(\w+)\s*(?:#\([\s.\w(\[\-:\]\),{'}`/+!~@#$%^&*=<>?]+\))?\s*[\w\[:\]]+\s*(?:\([\s.\w(\[\-:\]\),{'}`/+!~@#$%^&*=<>?]+\));"
    ).unwrap());
    let submodules: HashSet<String> = REGEX
        .captures_iter(contents) // Iterate over captures
        .filter_map(|caps| caps.ok()) // Filter out any failed captures
        .filter_map(|caps| caps.get(0)) // Get the first capture group
        .map(|m| m.as_str().split_whitespace().next().unwrap_or("").to_string()) // Split and get submodule name
        .filter(|s| !s.is_empty()) // Filter out any empty strings
        .collect(); // Collect into a HashSet
    
    if submodules.is_empty() {
        eprintln!("Warning: No submodules found in the content. This might be expected if the module doesn't use any submodules, or it could indicate an issue with the regex pattern or the content format.");
    } else {
        for submodule in &submodules {
            println!("Found submodule: {}", submodule);
        }
    }
    
    Ok(submodules)
}

pub fn include_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = Path::new(location).join(name_from_url(url));
    clone_repo(url, &repo_path).context("Failed to clone repository. Please check your internet connection and ensure you have git installed and properly configured.")?;
    Ok(())
}

pub fn clone_repo(url: &str, repo_path: &Path) -> Result<()> {
    let output = Command::new("git")
        .args([ "clone", "--depth", "1", "--single-branch", "--jobs", "4",
            url, repo_path.to_str().unwrap_or_default(),
        ])
        .output()
        .with_context(|| format!("Failed to execute git command. Please ensure git is installed and accessible in your PATH."))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git clone failed: {}. This could be due to network issues, invalid URL, or insufficient permissions. Please check your internet connection and the repository URL.", stderr));
    }

    Ok(())
}