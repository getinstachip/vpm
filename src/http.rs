use octocrab::Octocrab;

use crate::errors::CommandError::{self, *};

pub const REGISTRY_URL: &str = "https://github.com/";

pub struct HTTPRequest;
impl HTTPRequest {
    async fn registry(client: reqwest::Client, route: String) -> Result<String, CommandError> {
        let octocrab = Octocrab::builder().build()?;
        // client
        //     .get(format!("{}/{}", REGISTRY_URL, route))
        //     .send()
        //     .await
        //     .map_err(HTTPFailed)?
        //     .text()
        //     .await
        //     .or_else(|err| Err(FailedResponseText(err)))
    }

    pub async fn package_data(client: reqwest::Client, package_author: &String, package_name: &String) -> Result<PackageData, CommandError> {
        let response_raw = Self::registry(client, format!("/{}/{}", package_author, package_name));
        let body = response.text()?;
        let document = Html::parse_document(&body);
    }
}

