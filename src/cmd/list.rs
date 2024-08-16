use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::collections::HashSet;
use anyhow::Context;
use std::process::Command;
use crate::cmd::{Execute, List};
use tempfile::tempdir;

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";


impl Execute for List {
    fn execute(&self) -> Result<()> {
        let verilog_files = list_verilog_files()?;
        println!("Available Verilog modules:");
        for file in verilog_files {
            println!("  {}", file);
        }
        Ok(())
    }
}

fn list_verilog_files() -> Result<Vec<String>> {
    let temp_dir = tempdir()?;
    let repo_path = temp_dir.path();

    // Clone the repository
    Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--single-branch",
            "--jobs",
            "4",
            STD_LIB_URL,
            repo_path.to_str().unwrap_or_default(),
        ])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", STD_LIB_URL))?;

    let mut verilog_files = HashSet::new();

    // Walk through the repository
    for entry in walkdir::WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        if let Some(extension) = entry.path().extension() {
            if extension == "v" || extension == "sv" {
                if let Some(file_name) = entry.path().file_stem() {
                    verilog_files.insert(file_name.to_string_lossy().into_owned());
                }
            }
        }
    }

    Ok(verilog_files.into_iter().collect())
}
