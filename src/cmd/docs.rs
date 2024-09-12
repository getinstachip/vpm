use anyhow::{Result, Context, anyhow};
use reqwest::Client;
use std::path::PathBuf;
use serde_json::json;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

use crate::cmd::{Execute, Docs};
use crate::config_man::{decrypt_docs_count, encrypt_docs_count};

impl Execute for Docs {
    async fn execute(&self) -> Result<()> {
        let docs_count = decrypt_docs_count()?;
        if docs_count >= 10 {
            println!("You have used all your documentation generation credits. Consider upgrading to VPM Pro for unlimited and betterdocumentation generation.");
            return Ok(());
        }

        if self.from_repo {
            let content = fetch_module_content(&self.module_path).await
                .context("Failed to fetch module content. Please check your internet connection and ensure the provided URL is correct.")?;
            let file_name = self.module_path.split('/').last().unwrap_or(&self.module_path);
            let folder_name = file_name.split('.').next().unwrap_or(file_name);
            let destination = PathBuf::from("./vpm_modules").join(folder_name);
            fs::create_dir_all(&destination)
                .context("Failed to create destination directory. Please check if you have write permissions in the current directory.")?;
            if self.offline {
                generate_docs_offline(&self.module_path, &content, Some(destination.join(format!("{}_README.md", folder_name)))).await
                    .context("Failed to generate documentation offline. Please check the module content and try again.")?;
            } else {
                generate_docs(&self.module_path, &content, Some(destination.join(format!("{}_README.md", folder_name)))).await
                    .context("Failed to generate documentation. Please check the module content and try again.")?;
            }
        } else {
            let full_module_path = PathBuf::from(&self.module_path);
            
            if full_module_path.exists() {
                let content = fs::read_to_string(&full_module_path)
                    .with_context(|| format!("Failed to read module file: {}. Please ensure you have read permissions for this file.", full_module_path.display()))?;
                println!("Generating documentation for local module '{}'", self.module_path);
                let readme_path = full_module_path.with_file_name(format!("{}_README.md", full_module_path.file_stem().unwrap().to_str().unwrap()));
                if self.offline {
                    generate_docs_offline(&self.module_path, &content, Some(readme_path)).await
                        .context("Failed to generate documentation offline for the local module. Please check the module content and try again.")?;
                } else {
                    generate_docs(&self.module_path, &content, Some(readme_path)).await
                        .context("Failed to generate documentation for the local module. Please check the module content and try again.")?;
                }
            } else {
                return Err(anyhow!("Module '{}' not found in vpm_modules. Please provide a URL to a repository containing the module, or ensure the module exists in the correct location.", self.module_path));
            }
        }
        encrypt_docs_count(docs_count + 1)?;
        println!("Documentation generated successfully. You have used {} of your 10 credits.", docs_count + 1);
        Ok(())
    }
}

async fn fetch_module_content(url: &str) -> Result<String> {
    let client = reqwest::Client::new();

    // Extract the raw content URL
    let raw_url = url.replace("github.com", "raw.githubusercontent.com")
                     .replace("/blob/", "/");

    println!("Fetching content from URL: {}", raw_url);

    // Fetch the content
    let response = client.get(&raw_url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch module content: HTTP {}", response.status()));
    }

    let content = response.text().await?;

    Ok(content)
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

    let readme_path = if let Some(path) = full_module_path {
        path
    } else {
        let module_name = module_path.rsplit('/').next().unwrap_or(module_path);
        let dir = PathBuf::from("./vpm_modules").join(module_name).parent().unwrap().to_path_buf();
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

    // Check if Ollama is installed
    if !Command::new("ollama").arg("--version").output().is_ok() {
        pb.set_message("Ollama not found. Installing...");
        
        // Install Ollama
        let install_status = if cfg!(target_os = "macos") {
            Command::new("brew").args(&["install", "ollama"]).status()
        } else if cfg!(target_os = "linux") {
            Command::new("curl").args(&["-fsSL", "https://ollama.ai/install.sh", "|", "sh"]).status()
        } else {
            return Err(anyhow::anyhow!("Unsupported operating system for Ollama installation"));
        };

        if let Err(e) = install_status {
            return Err(anyhow::anyhow!("Failed to install Ollama: {}", e));
        }

        pb.set_message("Ollama installed successfully");
    }

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

    let readme_path = if let Some(path) = full_module_path {
        path
    } else {
        let module_name = module_path.rsplit('/').next().unwrap_or(module_path);
        let dir = PathBuf::from("./vpm_modules").join(module_name).parent().unwrap().to_path_buf();
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
