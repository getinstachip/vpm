use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("command '{0}' not found")]
    CommandNotFound(String),
    #[error("missing argument: '{0}'")]
    MissingArgument(String),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("failed to execute http request ({0})")]
    HTTPFailed(reqwest::Error),
    #[error("failed to get http response text ({0})")]
    FailedResponseText(reqwest::Error),
    #[error("failed to parse JSON ({0})")]
    JSONParseError(serde_json::Error),
    #[error("failed to write file ({0})")]
    IOError(std::io::Error),
    #[error("failed to connect to Elasticsearch ({0})")]
    ElasticsearchConnectionError(String),
    #[error("failed to get latest commit id ({0})")]
    FailedGetLatestCommitId(String),
    #[error("failed to update package ({0})")]
    FailedUpdatePackage(String),
    #[error("failed to update all packages")]
    FailedUpdateAllPackages(String),
    #[error("missing file ({0})")]
    MissingFile(String),
}
