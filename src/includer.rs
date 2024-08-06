use async_trait::async_trait;
use std::{fs, path::Path};
use crate::errors::ParseError;
use crate::errors::CommandError;
use crate::command_handler::CommandHandler;
use crate::http::HTTPRequest;
use regex::Regex;

#[derive(Debug, Default)]
pub struct Includer {
    module_name: String,
    repository: String,
}

impl Includer {
    pub fn new(module_name: String, repository: String) -> Self {
        Self {
            module_name,
            repository,
        }
    }

    // Helper function to parse module instantiations
    fn parse_module_instantiations(content: &str) -> Vec<String> {
        let re = regex::Regex::new(r"(\w+)\s*#?\s*\(([\s\S]*?)\)?\s*(\w+)\s*\(([\s\S]*?)\);").unwrap();
        let instances: Vec<String> = re.captures_iter(content)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect();
        instances
    }
}

#[async_trait]
impl CommandHandler for Includer {
    async fn execute(&self) -> Result<(), CommandError> {
       // loop through all files in repo recursively
        let client = reqwest::Client::new();

        // Split the repository string to get author and name
        let mut repo_parts = self.repository.split('/');
        let author = repo_parts.next().ok_or_else(|| CommandError::ParseError("Invalid repository format".to_string()))?;
        let name = repo_parts.next().ok_or_else(|| CommandError::ParseError("Invalid repository format".to_string()))?;

        let github_files = HTTPRequest::get_verilog_files(
            client.clone(),
            author.to_string(),
            name.to_string(),
        ).await?;

        // if verilog file matches module name
        for file in github_files {
            if let Some(download_url) = file.download_url {
                if file.name == self.module_name {
                    let content = client.get(&download_url)
                    .send()
                    .await
                    .map_err(CommandError::HTTPFailed)?
                    .text()
                    .await
                    .map_err(CommandError::FailedResponseText)?;               
                    // Parse the Verilog file content to extract module instantiations
                    let module_instances = Self::parse_module_instantiations(&content);
                    // Print the module name and its instantiated submodules
                    println!("Module: {}", self.module_name);
                    if module_instances.is_empty() {
                        println!("  No submodules instantiated.");
                    } else {
                        println!("  Instantiated submodules:");
                        for instance in module_instances {
                            println!("    - {}", instance);
                        }
                    }
                    // Break the loop as we've found and processed the matching file
                    break;
                }
            }
        }

        Ok(())
    }

    async fn list() -> Result<(), ParseError> {
        Ok(())
    }
}
