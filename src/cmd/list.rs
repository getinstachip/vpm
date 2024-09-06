use anyhow::{Result, Context, anyhow};
use std::collections::HashSet;
use std::process::Command;
use crate::cmd::{Execute, List};
use tempfile::tempdir;

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

impl Execute for List {
    async fn execute(&self) -> Result<()> {
        match list_verilog_files() {
            Ok(verilog_files) => {
                println!("Available Verilog modules:");
                for file in verilog_files {
                    println!("  {}", file);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: Failed to list Verilog files. {}", e);
                eprintln!("Debug steps:");
                eprintln!("1. Check your internet connection");
                eprintln!("2. Ensure git is installed and accessible from the command line");
                eprintln!("3. Verify you have read permissions for the temporary directory");
                Err(e)
            }
        }
    }
}

fn list_verilog_files() -> Result<Vec<String>> {
    let temp_dir = tempdir().context("Failed to create temporary directory. Ensure you have write permissions in the system temp directory.")?;
    let repo_path = temp_dir.path();

    // Clone the repository
    let output = Command::new("git")
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
        .output()
        .context("Failed to execute git command. Ensure git is installed and accessible from the command line.")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Git clone failed. Error: {}. This could be due to network issues, incorrect repository URL, or git configuration problems.",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let mut verilog_files = HashSet::new();

    for entry in walkdir::WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(extension) = entry.path().extension() {
            match extension.to_str() {
                Some("v") | Some("sv") => {
                    if let Some(file_name) = entry.path().file_stem() {
                        verilog_files.insert(file_name.to_string_lossy().into_owned());
                    }
                }
                _ => {}
            }
        }
    }

    if verilog_files.is_empty() {
        Err(anyhow!("No Verilog files found in the repository. This could indicate an issue with the repository structure or content."))
    } else {
        Ok(verilog_files.into_iter().collect())
    }
}
