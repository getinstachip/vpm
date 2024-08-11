use anyhow::{Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::runtime::Runtime;
use std::process::Command;

use clust::messages::{
    ClaudeModel,
    MaxTokens,
    Message,
    MessagesRequestBody,
    SystemPrompt
};
use clust::Client;

use crate::cmd::{Execute, Docs};

impl Execute for Docs {
    fn execute(&self) -> Result<()> {
        let rt = Runtime::new()?;
        rt.block_on(generate_docs(self.url.as_ref().unwrap(), &self.package_name))?;
        Ok(())
    }
}

async fn generate_docs(url: &str, module: &str) -> Result<()> {
    // Clone the repository
    println!("Cloning repository from {}", url);
    clone_repo(url, &format!("./vpm_modules/{}/", module))?;
    let file_contents = get_file_from_directory(&format!("./vpm_modules/{}/", module), module).unwrap_or_default();
    println!("Deleting cloned repository");
    fs::remove_dir_all(format!("./vpm_modules/{}/", module))?;
    let client = Client::from_env()?;
    let model = ClaudeModel::Claude35Sonnet20240620;
    let mut prompt = format!(
        "Please create a comprehensive Markdown documentation for the following Verilog module. Include an overview, module description, port list, parameters, and any important implementation details:\n\n{}",
        file_contents
    );
    prompt.push_str("\n\nPlease append a pinout diagram of the input ports on the left and output ports on the right.");
    let messages = vec![Message::user(prompt)];
    let max_tokens = MaxTokens::new(4096, model)?;
    let system_prompt = SystemPrompt::new("You are an expert Verilog engineer tasked with creating clear and detailed documentation.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    let response = client.create_a_message(request_body).await?;
    let response_content = response.content;

    let parsed_response: serde_json::Value = serde_json::from_str(&response_content.to_string())?;
    let generated_text = parsed_response[0]["text"].as_str().unwrap_or("");

    let readme_path = PathBuf::from("./").join(format!("{}_README.md", module));
    let mut readme_file = fs::File::create(&readme_path)?;

    writeln!(readme_file, "# {}", module)?;
    writeln!(readme_file)?;

    write!(readme_file, "{}", generated_text)?;

    println!("Documentation for {} written to {}", module, readme_path.display());

    Ok(())
}

fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
    Command::new("git")
        .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}

fn get_file_from_directory(dir_path: &str, file_name: &str) -> Option<String> {
    let path = std::path::Path::new(dir_path);
    if let Ok(entries) = path.read_dir() {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |f| f == file_name) {
                return std::fs::read_to_string(&path).ok();
            } else if path.is_dir() {
                if let Some(content) = get_file_from_directory(path.to_str().unwrap_or(""), file_name) {
                    return Some(content);
                }
            }
        }
    }
    None
}
