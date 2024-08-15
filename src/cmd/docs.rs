use anyhow::Result;
use reqwest::Client;
use tokio::runtime::Runtime;
use std::path::PathBuf;
use serde_json::json;

use crate::cmd::{Execute, Docs};

use super::install::install_module_from_url;

impl Execute for Docs {
    fn execute(&self) -> Result<()> {
        if let (Some(url), Some(module)) = (&self.url, &self.module) {
            println!("Parsing module '{}' from URL: '{}'", module, url);
            install_module_from_url(module, url)?;

            println!("Generating documentation for module '{}' from URL: '{}'", module, url);
            let rt = Runtime::new()?;
            rt.block_on(generate_docs(module))?;
        }
        Ok(())
    }
}

fn format_text(text: &str) -> String {
    text.replace("\\n", "\n")
        .replace("\\'", "'")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}

async fn generate_docs(module: &str) -> Result<()> {
    let dir = format!("./vpm_modules/{}", module);
    let file_path = format!("{}/{}", dir, module);
    let contents = tokio::fs::read_to_string(&file_path).await?;
    dbg!(&contents);

    let api_url = "https://bmniatl2bh.execute-api.us-east-1.amazonaws.com/dev/getApiKey";
    let client = Client::new();
    let response = client.post(api_url)
        .header("Content-Type", "application/json")
        .json(&json!({ "code": contents }))
        .send().await?;

    let documentation = format_text(&response.text().await?);

    let readme_path = PathBuf::from(&dir).join("README.md");
    tokio::fs::write(&readme_path, documentation).await?;

    println!("Documentation for {} written to {}", module, readme_path.display());

    Ok(())
}
