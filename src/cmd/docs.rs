use anyhow::{Result, Context, anyhow};
use reqwest::Client;
use std::path::PathBuf;
use serde_json::json;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use crate::cmd::include::clone_repo;
use std::process::{Command, Stdio};

use crate::cmd::{Execute, Docs};

impl Execute for Docs {
    async fn execute(&self) -> Result<()> {
        if let Some(url) = &self.url {
            let content = fetch_module_content(&self.module_path, url).await
                .context("Failed to fetch module content. Please check your internet connection and ensure the provided URL is correct.")?;
            if self.offline {
                generate_docs_offline(&self.module_path, &content, None).await
                    .context("Failed to generate documentation offline. Please check the module content and try again.")?;
            } else {
                generate_docs(&self.module_path, &content, None).await
                    .context("Failed to generate documentation. Please check the module content and try again.")?;
            }
        } else {
            let full_module_path = PathBuf::from(&self.module_path);
            
            if full_module_path.exists() {
                let content = fs::read_to_string(&full_module_path)
                    .with_context(|| format!("Failed to read module file: {}. Please ensure you have read permissions for this file.", full_module_path.display()))?;
                println!("Generating documentation for local module '{}'", self.module_path);
                if self.offline {
                    generate_docs_offline(&self.module_path, &content, Some(full_module_path)).await
                        .context("Failed to generate documentation offline for the local module. Please check the module content and try again.")?;
                } else {
                    generate_docs(&self.module_path, &content, Some(full_module_path)).await
                        .context("Failed to generate documentation for the local module. Please check the module content and try again.")?;
                }
            } else {
                return Err(anyhow!("Module '{}' not found in vpm_modules. Please provide a URL to a repository containing the module, or ensure the module exists in the correct location.", self.module_path));
            }
        }
        Ok(())
    }
}

async fn fetch_module_content(module_path: &str, url: &str) -> Result<String> {
    let tmp_dir = tempfile::tempdir()
        .context("Failed to create temporary directory. Please ensure you have write permissions in the system's temp directory.")?;
    let repo_path = tmp_dir.path();

    // Clone the repository using the public clone_repo function
    clone_repo(url, repo_path)
        .with_context(|| format!("Failed to clone repository from URL: {}. Please check your internet connection and ensure the URL is correct.", url))?;

    println!("Fetching content for module: {}", module_path);
    
    let module_file = repo_path.join(module_path);

    if !module_file.exists() {
        return Err(anyhow!("Module file not found in repository: {}. Please check if the module path is correct and exists in the repository.", module_path));
    }

    // Read the module content
    fs::read_to_string(&module_file)
        .with_context(|| format!("Failed to read module file: {}. Please ensure the file is not corrupted or empty.", module_file.display()))
}

fn format_text(text: &str) -> String {
    text.replace("\\n", "\n")
        .replace("\\'", "'")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}

async fn generate_docs(module_path: &str, content: &str, full_module_path: Option<PathBuf>) -> Result<()> {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    pb.set_position(33);
    pb.set_message("Generating documentation...");

    let client = Client::new();
    let api_url = "https://bmniatl2bh.execute-api.us-east-1.amazonaws.com/dev/getApiKey";
    let response = client.post(api_url)
        .header("Content-Type", "application/json")
        .json(&json!({ "code": content }))
        .send().await
        .context("Failed to send request to documentation generation API. Please check your internet connection and try again.")?;

    let documentation = format_text(&response.text().await
        .context("Failed to read response from documentation generation API. The API might be experiencing issues. Please try again later.")?);

    pb.set_position(66);
    pb.set_message("Writing documentation to file...");

    let module_name = module_path.rsplit('/').next().unwrap_or(module_path);
    let readme_path = if let Some(path) = full_module_path {
        path.with_file_name(format!("{}_README.md", module_name))
    } else {
        let dir = PathBuf::from("./vpm_modules").join(module_path).parent().unwrap().to_path_buf();
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create directory: {}. Please ensure you have write permissions in this location.", dir.display()))?;
        dir.join(format!("{}_README.md", module_name))
    };
    tokio::fs::write(&readme_path, documentation).await
        .with_context(|| format!("Failed to write documentation to file: {}. Please ensure you have write permissions in this location.", readme_path.display()))?;
    
    pb.set_position(100);
    pb.finish_with_message(format!("Documentation for {} written to {}", module_path, readme_path.display()));

    Ok(())
}

async fn generate_docs_offline(module_path: &str, content: &str, full_module_path: Option<PathBuf>) -> Result<()> {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    pb.set_position(33);
    pb.set_message("Generating documentation offline...");

    // Start Ollama server in the background
    let mut ollama_serve = Command::new("ollama")
        .arg("serve")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to start Ollama server. Make sure it's installed and in your PATH.")?;

    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Prepare the Ollama command
    let ollama_output = Command::new("ollama")
        .arg("run")
        .arg("codellama")
        .arg("Generate documentation for the following Verilog module:")
        .arg(content)
        .output()
        .context("Failed to execute Ollama. Make sure it's installed and in your PATH.")?;

    // Stop the Ollama server
    ollama_serve.kill().context("Failed to stop Ollama server")?;

    if !ollama_output.status.success() {
        return Err(anyhow::anyhow!("Ollama command failed: {}", String::from_utf8_lossy(&ollama_output.stderr)));
    }

    let documentation = String::from_utf8(ollama_output.stdout)
        .context("Failed to parse Ollama output as UTF-8")?;

    pb.set_position(66);
    pb.set_message("Writing documentation to file...");

    let module_name = module_path.rsplit('/').next().unwrap_or(module_path);
    let readme_path = if let Some(path) = full_module_path {
        path.with_file_name(format!("{}_README.md", module_name))
    } else {
        let dir = PathBuf::from("./vpm_modules").join(module_path).parent().unwrap().to_path_buf();
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create directory: {}. Please ensure you have write permissions in this location.", dir.display()))?;
        dir.join(format!("{}_README.md", module_name))
    };
    tokio::fs::write(&readme_path, documentation).await
        .with_context(|| format!("Failed to write documentation to file: {}. Please ensure you have write permissions in this location.", readme_path.display()))?;
    
    pb.set_position(100);
    pb.finish_with_message(format!("Documentation for {} written to {}", module_path, readme_path.display()));

    Ok(())
}
