use async_trait::async_trait;
use std::{fs, path::Path};
use crate::errors::ParseError;
use crate::errors::CommandError;
use crate::command_handler::CommandHandler;
use crate::http::GitHubFile;
use crate::http::HTTPRequest;
use tree_sitter::Parser;

use crate::headers::generate_header;

#[derive(Debug, Default)]
pub struct Includer {
    module_name: String,
    repository: String,
    local: bool,
}

impl Includer {
    pub fn new(module_name: String, repository: String, local: bool) -> Self {
        Self {
            module_name,
            repository,
            local,
        }
    }

    fn parse_module_hierarchy(content: &str) -> Vec<String> {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_verilog::language()).expect("Error loading Verilog grammar");
        
        let tree = parser.parse(content, None).expect("Failed to parse Verilog content");
        let root_node = tree.root_node();
        
        let mut module_names = Vec::new();
        Self::traverse_tree(&root_node, content, &mut module_names);

        // Print module names
        for module_name in &module_names {
            println!("Module: {}", module_name);
        }
        
        module_names
    }
    fn traverse_tree(node: &tree_sitter::Node, source: &str, instances: &mut Vec<String>) {
        // if node.kind() == "class_identifier" {
            // println!("Node: {:?}", node.kind());
            // if let Ok(node_text) = node.utf8_text(source.as_bytes()) {
                // println!("Node content: {}", node_text);
            // }
        // }

        if node.kind() == "module_instantiation" {
            if let Some(first_child) = node.child(0) {
                if let Ok(module_name) = first_child.utf8_text(source.as_bytes()) {
                    instances.push(format!("{}.v", module_name));
                }
            }
        }
        
        for child in node.children(&mut node.walk()) {
            // if child.start_position().row < 1718 {
                Self::traverse_tree(&child, source, instances);
            // }
        }
    }

    async fn process_files(&self, client: reqwest::Client, github_files: Vec<GitHubFile>) -> Result<(), CommandError> {
        let mut visited_files = std::collections::HashSet::new();
        let mut files_to_process = vec![self.module_name.clone()];
        let vpm_modules_dir = std::path::Path::new("vpm_modules").join(self.module_name.trim_end_matches(".v"));
        let vpm_modules_path = vpm_modules_dir.as_path();
        if !vpm_modules_path.exists() {
            std::fs::create_dir_all(vpm_modules_path)
                .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to create vpm_modules directory")))?;
        }
        let toml_file_name = format!("{}.toml", self.module_name.trim_end_matches(".v"));
        let toml_file_path = vpm_modules_path.join(&toml_file_name);
        let mut toml_content = format!(
            "[package]\n\
            name = \"{}\"\n\
            repository = \"{}\"\n\n\
            [dependencies]\n",
            self.module_name.trim_end_matches(".v"),
            self.repository
        );

        std::fs::write(&toml_file_path, &toml_content)
            .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to create {} file", toml_file_name)))?;

        while let Some(current_file) = files_to_process.pop() {
            if visited_files.contains(&current_file) {
                continue;
            }

            if let Some(file) = github_files.iter().find(|f| f.name == current_file) {
                if let Some(download_url) = &file.download_url {
                    let content = client.get(download_url)
                        .send()
                        .await
                        .map_err(CommandError::HTTPFailed)?
                        .text()
                        .await
                        .map_err(CommandError::FailedResponseText)?;

                    let file_path = vpm_modules_dir.join(&current_file);
                    std::fs::write(&file_path, &content)
                        .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to write file {}", file_path.display())))?;
                    // Generate and write header file
                    let header_content = generate_header(&content, &current_file);
                    let header_file_name = format!("{}.vh", current_file.trim_end_matches(".v"));
                    let header_file_path = vpm_modules_dir.join(&header_file_name);
                    std::fs::write(&header_file_path, &header_content)
                        .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to write header file {}", header_file_path.display())))?;

                    println!("Generated header file: {}", header_file_name);

                    let instances = Self::parse_module_hierarchy(&content);
                    for instance in instances {
                        if !visited_files.contains(&instance) {
                            files_to_process.push(instance.clone());
                            toml_content.push_str(&format!("{} = \"*\"\n", instance.trim_end_matches(".v")));
                        }
                    }

                    visited_files.insert(current_file);
                }
            }
        }

        std::fs::write(&toml_file_path, &toml_content)
            .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to update {} file", toml_file_name)))?;

        println!("Updated {} file with dependencies", toml_file_name);

        Ok(())
    }

    fn find_constraint_files(&self, github_files: Vec<GitHubFile>, module_name: &str) -> Vec<GitHubFile> {
        let constraint_extensions = [".sdc", ".xdc", ".ucd"];
        let module_name_without_extension = module_name.trim_end_matches(".v");

        github_files
            .iter()
            .filter(|file| {
                let file_name_without_extension = file.name.rsplit('.').next().unwrap_or(&file.name);
                file_name_without_extension == module_name_without_extension
                    && constraint_extensions.iter().any(|&ext| file.name.ends_with(ext))
            })
            .cloned()
            .collect()
    }

    async fn download_constraint_files(&self, client: reqwest::Client, constraint_files: Vec<GitHubFile>, module_name: &str) -> Result<(), CommandError> {
        let vpm_modules_dir = Path::new("vpm_modules").join(module_name).join("constraints");
        fs::create_dir_all(&vpm_modules_dir)
            .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to create directory {}", vpm_modules_dir.display())))?;

        for file in constraint_files {
            if let Some(download_url) = file.download_url {
                let content = client
                    .get(&download_url)
                    .send()
                    .await
                    .map_err(CommandError::HTTPFailed)?
                    .text()
                    .await
                    .map_err(CommandError::FailedResponseText)?;

                let file_path = vpm_modules_dir.join(&file.name);
                fs::write(&file_path, &content)
                    .map_err(|_| CommandError::WriteToVpmModulesError(format!("Failed to write file {}", file_path.display())))?;

                println!("Downloaded constraint file: {}", file.name);
            }
        }

        Ok(())
    }

    pub async fn process_author_repos(&self, author: &str) -> Result<(), CommandError> {
        let client = reqwest::Client::new();
        let repos = HTTPRequest::get_author_repos(client.clone(), author.to_string()).await?;

        for repo in repos {
            println!("Processing repository: {}", repo);
            let github_files = HTTPRequest::get_verilog_files(
                client.clone(),
                author.to_string(),
                repo.clone(),
            ).await?;
            
            self.process_files(client.clone(), github_files).await?;
        }

        Ok(())
    }

}

#[async_trait]
impl CommandHandler for Includer {
    async fn execute(&self) -> Result<(), CommandError> {
        let mut repo_parts = self.repository.split('/');
        let author = repo_parts.next().ok_or_else(|| CommandError::ParseError("Invalid repository format".to_string()))?;
        let name = repo_parts.next().ok_or_else(|| CommandError::ParseError("Invalid repository format".to_string()))?;
        let client = reqwest::Client::new();
        if !self.local {
            self.process_author_repos(author).await?
        } else {
            let github_files = HTTPRequest::get_verilog_files(
                client.clone(),
                author.to_string(),
                name.to_string(),
            ).await?;
            let github_files_clone = github_files.clone();
            self.process_files(client.clone(), github_files).await?;
            let constraint_files = self.find_constraint_files(
                github_files_clone,
                name,
            );
            self.download_constraint_files(client, constraint_files, self.module_name.clone().as_str()).await?;
        }

        Ok(())
    }

    async fn list() -> Result<(), ParseError> {
        Ok(())
    }
}