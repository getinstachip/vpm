use anyhow::Result;
use std::path::PathBuf;
use tokio::runtime::Runtime;

use clust::messages::{
    ClaudeModel,
    MaxTokens,
    Message,
    MessagesRequestBody,
    SystemPrompt
};
use clust::Client;

use crate::cmd::{Execute, Docs};

use super::install::install_module_from_url;

impl Execute for Docs {
    fn execute(&self) -> Result<()> {
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Parsing module information...");
            install_module_from_url(name, url, false)?;

            let rt = Runtime::new()?;
            rt.block_on(generate_docs(name))?;
        }

        Ok(())
    }
}

async fn generate_docs(module: &str) -> Result<()> {
    println!("Generating documentation...");

    let dir = format!("./vpm_modules/{}", module);
    let file_path = format!("{}/{}", dir, module);
    let contents = tokio::fs::read_to_string(&file_path).await;

    let client = Client::from_env()?;
    let model = ClaudeModel::Claude35Sonnet20240620;
    
    let messages = vec![
        Message::user(format!(
            "Please create a comprehensive Markdown documentation for the following Verilog module:

            {:#?}

            Include the following sections:
            1. Overview and module description
            2. Pinout diagram with input ports on the left and output ports on the right (do not show width of multi-bit ports)
            3. Table of ports
            4. Table of parameters
            5. Important implementation details
            6. List of any major bugs or caveats (if they exist)

            This last point about bugs and caveats is extremely important to include if applicable.",

            contents
        )),
    ];

    let max_tokens = MaxTokens::new(2048, model)?;
    let system_prompt = SystemPrompt::new("You are an expert Verilog engineer tasked with creating clear and detailed documentation.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    let (response, _) = tokio::join!(
        client.create_a_message(request_body),
        tokio::fs::create_dir_all(&dir)
    );

    let response = response?;
    let generated_text = response.content.flatten_into_text().unwrap_or("Error generating documentation");

    let readme_path = PathBuf::from(&dir).join("README.md");
    tokio::fs::write(&readme_path, format!("```{}```\n\n{}", module, generated_text)).await?;

    println!("Documentation for {} written to {}", module, readme_path.display());

    Ok(())
}
