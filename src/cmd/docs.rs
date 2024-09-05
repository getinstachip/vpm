use anyhow::{Result, Context};
use reqwest::Client;
use std::path::PathBuf;
use serde_json::json;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use crate::cmd::include::clone_repo;

use crate::cmd::{Execute, Docs};

impl Execute for Docs {
    async fn execute(&self) -> Result<()> {
        if let Some(url) = &self.url {
            let content = fetch_module_content(&self.module_path, url).await?;
            generate_docs(&self.module_path, &content, None).await?;
        } else {
            let full_module_path = PathBuf::from(&self.module_path);
            
            if full_module_path.exists() {
                let content = fs::read_to_string(&full_module_path)
                    .context(format!("Failed to read module file: {}", full_module_path.display()))?;
                println!("Generating documentation for local module '{}'", self.module_path);
                generate_docs(&self.module_path, &content, Some(full_module_path)).await?;
            } else {
                println!("Module '{}' not found in vpm_modules. Please provide a URL to a repository containing the module.", self.module_path);
            }
        }
        Ok(())
    }
}

async fn fetch_module_content(module_path: &str, url: &str) -> Result<String> {
    let tmp_dir = tempfile::tempdir()?;
    let repo_path = tmp_dir.path();

    // Clone the repository using the public clone_repo function
    clone_repo(url, repo_path)?;

    println!("Fetching content for module: {}", module_path);
    
    let module_file = repo_path.join(module_path);

    if !module_file.exists() {
        return Err(anyhow::anyhow!("Module file not found in repository: {}", module_path));
    }

    // Read the module content
    let content = fs::read_to_string(module_file)?;

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
        .unwrap()
        .progress_chars("#>-"));
    
    pb.set_position(33);
    pb.set_message("Generating documentation...");

    let client = Client::new();
    let api_url = "https://bmniatl2bh.execute-api.us-east-1.amazonaws.com/dev/getApiKey";
    let response = client.post(api_url)
        .header("Content-Type", "application/json")
        .json(&json!({ "code": content }))
        .send().await?;


    // println!("Response: {}", &response.text().await?);
    let documentation = format_text(&response.text().await?);

    pb.set_position(66);
    pb.set_message("Writing documentation to file...");

    let module_name = module_path.rsplit('/').next().unwrap_or(module_path);
    let readme_path = if let Some(path) = full_module_path {
        path.with_file_name(format!("{}_README.md", module_name))
    } else {
        let dir = PathBuf::from("./vpm_modules").join(module_path).parent().unwrap().to_path_buf();
        fs::create_dir_all(&dir)?;
        dir.join(format!("{}_README.md", module_name))
    };
    tokio::fs::write(&readme_path, documentation).await?;
    
    pb.set_position(100);
    pb.finish_with_message(format!("Documentation for {} written to {}", module_path, readme_path.display()));

    Ok(())
}