use crate::errors::CommandError::{self, *};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
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
            .header("User-Agent", "vpm")
            .header(
                "Authorization",
                format!("token {}", std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set")),
            )
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
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
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

    pub async fn get_latest_commit_id(
        client: Client,
        owner: String,
        repo: String,
    ) -> Result<String, CommandError> {
        let route = format!("repos/{}/{}/commits", owner, repo);
        let response_raw = Self::api_request(client, route).await?;
        // First, try to parse as a single commit object
        if let Ok(commit) = serde_json::from_str::<serde_json::Value>(&response_raw) {
            if let Some(commit_sha) = commit.get("sha").and_then(|sha| sha.as_str()) {
                return Ok(commit_sha.to_string());
            }
        }
        // If that fails, try to parse as an array of commits
        if let Ok(commits) = serde_json::from_str::<Vec<serde_json::Value>>(&response_raw) {
            if let Some(first_commit) = commits.first() {
                if let Some(commit_sha) = first_commit.get("sha").and_then(|sha| sha.as_str()) {
                    return Ok(commit_sha.to_string());
                }
            }
        }
        // If both parsing attempts fail, check for rate limit error
        if response_raw.contains("API rate limit exceeded") {
            return Err(CommandError::FailedGetLatestCommitId(
                "GitHub API rate limit exceeded. Please try again later or use authentication.".to_string()
            ));
        }
        // If we reach here, we couldn't parse the response or find a commit SHA
        Err(CommandError::FailedGetLatestCommitId(
            format!("Failed to parse response or find commit SHA. Raw response: {}", response_raw)
        ))
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

        let items: Vec<GitHubFile> = serde_json::from_str(&response_raw).map_err(JSONParseError)?;
        let mut all_files = Vec::new();

        for item in &items {
            all_files.push(item.clone());
            if item.download_url.is_none() {
                let sub_files = Box::pin(Self::get_verilog_files_recursive(
                    client.clone(),
                    owner,
                    repo,
                    &item.path,
                    pb.clone(),
                ))
                .await?;
                all_files.extend(sub_files);
            }
        }

        Ok(all_files)
    }
}
