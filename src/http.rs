use octocrab::Octocrab;
use octocrab::Error as OctocrabError;

use crate::errors::CommandError::{self, *};

pub const REGISTRY_URL: &str = "https://github.com/";

pub struct HTTPRequest;
impl HTTPRequest {
    pub fn parse_github_url(url: &str) -> Result<(String, String), CommandError> {
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
        
        if parts.len() < 5 || parts[2] != "github.com" {
            return Err(CommandError::InvalidArgument("Invalid GitHub URL format".to_string()));
        }

        let author = parts[3].to_string();
        let name = parts[4].to_string();

        Ok((author, name))
    }
    async fn registry(client: reqwest::Client, github_url: String) -> Result<(), CommandError> {
        let (package_author, package_name) = Self::parse_github_url(&github_url)?;
        let octocrab = Octocrab::builder()
    .build()
    .map_err(|e| CommandError::HTTPFailed(format!("Failed to build Octocrab client: {}", e)))?;
    let repo = octocrab.repos(&package_author, &package_name);
        let contents = repo.get_content().path("").send().await.map_err(|e| CommandError::HTTPFailed(e.to_string()))?;

        for item in contents.items {
            if item.name.ends_with(".v") {
                let file_content = octocrab.repos(&package_author, &package_name)
                    .get_content()
                    .path(&item.path)
                    .send()
                    .await
                    .map_err(|e| CommandError::HTTPFailed(e.to_string()))?;

                let decoded_content = file_content.download_url.unwrap();
                let file_path = std::path::Path::new("vpm_modules").join(&item.name);
                std::fs::create_dir_all("vpm_modules").map_err(|e| CommandError::IOError(e.to_string()))?;
                
                let response = client.get(&decoded_content).send().await
                    .map_err(|e| CommandError::HTTPFailed(e.to_string()))?;
                let content = response.bytes().await
                    .map_err(|e| CommandError::HTTPFailed(e.to_string()))?;
                
                std::fs::write(file_path, content).map_err(|e| CommandError::IOError(e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn package_data(client: reqwest::Client, package_author: &String, package_name: &String) -> Result<(), CommandError> {
        Self::registry(client, format!("/{}/{}", package_author, package_name)).await
    }
}