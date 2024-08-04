use crate::errors::CommandError::{self, *};
use reqwest::Client;
use serde::Deserialize;

pub const GITHUB_API_URL: &str = "https://api.github.com";

#[derive(Deserialize, Debug)]
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
                format!("token {}", "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"),
            )
            .header("User-Agent", "rithvikru")
            .send()
            .await
            .map_err(HTTPFailed)?
            .text()
            .await
            .map_err(FailedResponseText)
    }

    pub async fn get_verilog_files(
        client: Client,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitHubFile>, CommandError> {
        Self::get_verilog_files_recursive(client, owner, repo, "").await
    }

    async fn get_verilog_files_recursive(
        client: Client,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Vec<GitHubFile>, CommandError> {
        let response_raw = Self::api_request(
            client.clone(),
            format!("repos/{}/{}/contents/{}", owner, repo, path),
        )
        .await?;

        let items: Vec<GitHubFile> = serde_json::from_str(&response_raw).map_err(JSONParseError)?;
        let mut verilog_files = Vec::new();

        for item in items {
            if item.name.ends_with(".v") {
                verilog_files.push(item);
            } else if item.download_url.is_none() {
                // This is a directory, recursively search it
                let sub_files = Box::pin(Self::get_verilog_files_recursive(
                    client.clone(),
                    owner,
                    repo,
                    &item.path,
                ))
                .await?;
                verilog_files.extend(sub_files);
            }
        }

        Ok(verilog_files)
    }
}
