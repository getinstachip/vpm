use crate::errors::CommandError::{self, *};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub const GITHUB_API_URL: &str = "https://api.github.com";

#[derive(Deserialize, Debug, Clone)]
pub struct GitHubFile {
    pub name: String,
    pub path: String,
    pub download_url: Option<String>,
}

pub struct HTTPRequest;
impl HTTPRequest {
    async fn api_request(client: Client, route: String) -> Result<String, CommandError> {
        client
            .get(format!("{}/{}", GITHUB_API_URL, route))
            .header("Accept", "application/vnd.github.v3+json")
            .header(
                "Authorization",
                format!("token {}", env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set")),
            )
            .header("User-Agent", "vpm")
            .send()
            .await
            .map_err(HTTPFailed)?
            .text()
            .await
            .map_err(FailedResponseText)
    }

    pub async fn get_verilog_files(
        client: Client,
        owner: String,
        repo: String,
    ) -> Result<Vec<GitHubFile>, CommandError> {
        println!(
            "Parsing repository: https://github.com/{}/{}..",
            owner, repo
        );
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message("Parsing repository structure...");
        pb.enable_steady_tick(Duration::from_millis(100));

        let pb = Arc::new(pb);
        let verilog_files =
            Self::get_verilog_files_recursive(client, &owner, &repo, "", pb.clone()).await?;

        pb.finish_with_message("✨ Repository structure parsed successfully!");
        Ok(verilog_files)
    }

    async fn get_verilog_files_recursive(
        client: Client,
        owner: &str,
        repo: &str,
        path: &str,
        pb: Arc<ProgressBar>,
    ) -> Result<Vec<GitHubFile>, CommandError> {
        pb.set_message(format!("Parsing: {}", path));
        sleep(Duration::from_millis(10)).await;

        let response_raw = Self::api_request(
            client.clone(),
            format!("repos/{}/{}/contents/{}", owner, repo, path),
        )
        .await?;

        let items: Vec<GitHubFile> =
            serde_json::from_str(&response_raw).map_err(JSONParseError)?;
        let mut verilog_files = Vec::new();

        for item in &items {
            if item.name.ends_with(".v") {
                verilog_files.push(item.clone());
            } else if item.download_url.is_none() {
                let sub_files = Box::pin(Self::get_verilog_files_recursive(
                    client.clone(),
                    owner,
                    repo,
                    &item.path,
                    pb.clone(),
                ))
                .await?;
                verilog_files.extend(sub_files);
            }
        }

        Ok(verilog_files)
    }
}