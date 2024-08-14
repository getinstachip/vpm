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
use clust::{ApiKey, Client};
use serde_json::Value;

use aws_sdk_secretsmanager::config::{Credentials, Region};
use aws_config::BehaviorVersion;

use crate::cmd::{Execute, Docs};

use super::install::install_module_from_url;

impl Execute for Docs {
    fn execute(&self) -> Result<()> {
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Parsing module information...");
            install_module_from_url(name, url, false)?;

            let rt = Runtime::new()?;
            let api_key: Value = serde_json::from_str(&rt.block_on(get_api_key())?)?;
            rt.block_on(generate_docs(name, api_key["Value"].as_str().unwrap()))?;
        }

        Ok(())
    }
}

async fn get_api_key() -> Result<String, aws_sdk_secretsmanager::Error> {
    let region = "us-east-1";
    let access_key_id = "AKIA4SDNVWJVBYSA4BKA";
    let secret_access_key = "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";

    let secret_name = "ANTHROPIC_API_KEY";

    // Create credentials
    let credentials = Credentials::new(
        access_key_id,
        secret_access_key,
        None,
        None,
        "env_credentials",
    );

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(Region::new(region))
        .credentials_provider(credentials)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;

    let secret = response.secret_string().unwrap();

    Ok(secret.to_string())
}

async fn generate_docs(module: &str, api_key: &str) -> Result<()> {
    println!("Generating documentation...");

    let dir = format!("./vpm_modules/{}", module);
    let file_path = format!("{}/{}", dir, module);
    let contents = tokio::fs::read_to_string(&file_path).await;

    // Use the embedded API key
    let client = Client::from_api_key(ApiKey::new(api_key));
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
