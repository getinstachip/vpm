use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, Write};
use anyhow::Context;

use crate::cmd::{Execute, Dotf};

impl Execute for Dotf {
    fn execute(&self) -> Result<()> {
        append_modules_to_filelist(&self.path_to_top_module, true);
        Ok(())
    }
}

pub fn append_modules_to_filelist(top_module_path: &str, sub: bool) -> Result<()> {
    let vpm_modules_dir = PathBuf::from("./vpm_modules");
    let mut visited_modules: Vec<String> = Vec::new();

    let top_module_file = Path::new(top_module_path).file_name().and_then(|f| f.to_str()).unwrap_or("");
    let top_module_dir = Path::new(top_module_path).with_extension("").to_str().unwrap_or("").to_string();
    let filelist_name = format!("{}.f", top_module_file.trim_end_matches(".sv").trim_end_matches(".v"));
    let filelist_path = PathBuf::from("vpm_modules").join(top_module_dir).join(filelist_name);
    // println!("Filelist path: {}", filelist_path.display());

    append_module(&vpm_modules_dir, top_module_file, top_module_file, &mut visited_modules, sub, &filelist_path)?;

    fn append_module(
        dir: &Path,
        module: &str,
        top_module: &str,
        visited_modules: &mut Vec<String>,
        sub: bool,
        filelist_path: &Path,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |name| name == module) {
                let module_path = path.to_str().unwrap_or_default();
                fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(filelist_path)?
                    .write_all(format!("{}\n", module_path).as_bytes())?;

                if sub {
                    let contents = fs::read_to_string(&path)?;
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
                            filelist_path)?;
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
                    filelist_path)?;
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
                                filelist_path)?;
                            append_module(
                                &PathBuf::from("./vpm_modules"),
                                &module_name_sv,
                                top_module,
                                visited_modules,
                                sub,
                                filelist_path)?;
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
                filelist_path)?;
        }
        
        Ok(())
    }

    Ok(())
}