use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

use crate::cmd::{Execute, Dotf};

impl Execute for Dotf {
    async fn execute(&self) -> Result<()> {
        // Clear the .f file if it already exists
        let top_module_file = Path::new(&self.path_to_top_module).file_name().and_then(|f| f.to_str()).unwrap_or("");
        let top_module_dir = Path::new(&self.path_to_top_module).with_extension("").to_str().unwrap_or("").to_string();
        let filelist_name = format!("{}.f", top_module_file.trim_end_matches(".sv").trim_end_matches(".v"));
        let filelist_path = PathBuf::from("vpm_modules").join(&top_module_dir).join(&filelist_name);

        if filelist_path.exists() {
            fs::write(&filelist_path, "")?;
        }
        let _ = append_modules_to_filelist(&self.path_to_top_module, true);
        Ok(())
    }
}
pub fn append_modules_to_filelist(top_module_path: &str, sub: bool) -> Result<()> {
    let vpm_modules_dir = PathBuf::from("./vpm_modules");
    let mut visited_modules: Vec<String> = Vec::new();

    let top_module_file = Path::new(top_module_path).file_name().and_then(|f| f.to_str()).unwrap_or("");
    let top_module_dir = Path::new(top_module_path).with_extension("").to_str().unwrap_or("").to_string();
    let filelist_name = format!("{}.f", top_module_file.trim_end_matches(".sv").trim_end_matches(".v"));
    let filelist_path = PathBuf::from("vpm_modules").join(&top_module_dir).join(&filelist_name);

    let mut filepaths = Vec::new();
    let mut f_statements = Vec::new();
    let mut define_statements = Vec::new();

    append_module(&vpm_modules_dir, top_module_file, top_module_file, &mut visited_modules, sub, &filelist_path, &mut filepaths, &mut f_statements, &mut define_statements)?;

    // Write all filepaths together
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filelist_path)?;

    // Add +incdir+ statement
    file.write_all(format!("+incdir+{}\n\n", vpm_modules_dir.join(&top_module_dir).display()).as_bytes())?;

    for filepath in filepaths {
        file.write_all(format!("{}\n", filepath).as_bytes())?;
    }

    file.write_all(b"\n")?;

    // Write all unique define statements together
    let mut unique_defines: std::collections::HashSet<String> = std::collections::HashSet::new();
    for define in define_statements {
        unique_defines.insert(define);
    }
    for define in unique_defines {
        file.write_all(format!("{}\n", define).as_bytes())?;
    }

    file.write_all(b"\n")?;

    // Write all -f statements together
    for f_statement in f_statements {
        file.write_all(format!("{}\n", f_statement).as_bytes())?;
    }

    Ok(())
}

fn append_module(
    dir: &Path,
    module: &str,
    top_module: &str,
    visited_modules: &mut Vec<String>,
    sub: bool,
    filelist_path: &Path,
    filepaths: &mut Vec<String>,
    f_statements: &mut Vec<String>,
    define_statements: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == module) {
            let module_path = path.to_str().unwrap_or_default();
            let contents = fs::read_to_string(&path)?;

            filepaths.push(module_path.to_string());

            // Check for `define macros and module boundaries
            let mut in_ifdef_block = false;
            let mut in_module = false;
            let mut module_defines = Vec::new();
            let mut current_module_name = String::new();

            for line in contents.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with("module") {
                    in_module = true;
                    current_module_name = trimmed_line.split_whitespace().nth(1).unwrap_or("").to_string();
                } else if trimmed_line.starts_with("endmodule") {
                    in_module = false;
                    if !module_defines.is_empty() {
                        if current_module_name == top_module.trim_end_matches(".v") {
                            // Add defines to the top module's .f file
                            define_statements.extend(module_defines.clone());
                        } else {
                            let submodule_filelist_name = format!("{}.f", current_module_name);
                            let submodule_filelist_path = dir.join(&submodule_filelist_name);
                            let mut submodule_file = fs::OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open(&submodule_filelist_path)?;
                            for define in &module_defines {
                                submodule_file.write_all(format!("{}\n", define).as_bytes())?;
                            }
                            f_statements.push(format!("-f {}", submodule_filelist_path.to_str().unwrap_or_default()));
                        }
                    }
                    module_defines.clear();
                } else if trimmed_line.starts_with("`ifdef") {
                    in_ifdef_block = true;
                    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let macro_name = parts[1];
                        let define = format!("+define+{}", macro_name);
                        if !in_module {
                            if !define_statements.contains(&define) {
                                define_statements.push(define.clone());
                            }
                        } else {
                            module_defines.push(define);
                        }
                    }
                } else if trimmed_line.starts_with("`endif") {
                    in_ifdef_block = false;
                } else if !in_ifdef_block && trimmed_line.starts_with("`define") {
                    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                    let define = if parts.len() >= 3 {
                        let macro_name = parts[1];
                        let macro_value = parts[2..].join(" ");
                        format!("+define+{}={}", macro_name, macro_value)
                    } else if parts.len() == 2 {
                        let macro_name = parts[1];
                        format!("+define+{}", macro_name)
                    } else {
                        continue;
                    };
                    if !in_module {
                        if !define_statements.contains(&define) {
                            define_statements.push(define.clone());
                        }
                    } else {
                        module_defines.push(define);
                    }
                }
            }

            if sub {
                let mut parser = tree_sitter::Parser::new();
                parser
                    .set_language(tree_sitter_verilog::language())
                    .expect("Error loading Verilog grammar");

                if let Some(tree) = parser.parse(&contents, None) {
                    let root_node = tree.root_node();
                    find_module_instantiations(
                        root_node,
                        top_module,
                        &contents,
                        visited_modules,
                        sub,
                        filelist_path,
                        filepaths,
                        f_statements,
                        define_statements)?;
                }
            }

            return Ok(());
        } else if path.is_dir() {
            append_module(
                &path,
                module,
                top_module,
                visited_modules,
                sub,
                filelist_path,
                filepaths,
                f_statements,
                define_statements)?;
        }
    }

    Ok(())
}

fn find_module_instantiations(
    root_node: tree_sitter::Node,
    top_module: &str,
    contents: &str,
    visited_modules: &mut Vec<String>,
    sub: bool,
    filelist_path: &Path,
    filepaths: &mut Vec<String>,
    f_statements: &mut Vec<String>,
    define_statements: &mut Vec<String>,
) -> Result<()> {
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        if child.kind().contains("instantiation") {
            if let Some(first_child) = child.child(0) {
                if let Ok(module) = first_child.utf8_text(contents.as_bytes()) {
                    let module_name_v = format!("{}.v", module);
                    let module_name_sv = format!("{}.sv", module);
                    if !visited_modules.contains(&module_name_v) && !visited_modules.contains(&module_name_sv) {
                        visited_modules.push(module_name_v.clone());
                        visited_modules.push(module_name_sv.clone());
                        append_module(
                            &PathBuf::from("./vpm_modules"),
                            &module_name_v,
                            top_module,
                            visited_modules,
                            sub,
                            filelist_path,
                            filepaths,
                            f_statements,
                            define_statements)?;
                        append_module(
                            &PathBuf::from("./vpm_modules"),
                            &module_name_sv,
                            top_module,
                            visited_modules,
                            sub,
                            filelist_path,
                            filepaths,
                            f_statements,
                            define_statements)?;
                    }
                }
            }
        }

        find_module_instantiations(
            child,
            top_module,
            contents,
            visited_modules,
            sub,
            filelist_path,
            filepaths,
            f_statements,
            define_statements)?;
    }
    
    Ok(())
}
