use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use anyhow::bail;
use chatgpt::prelude::*;
use regex::Regex;

use crate::cmd::{Execute, Install};

const LOCATION: &str = "./vpm_modules";
const STD_LIB_URL: &str = "https://github.com/vlang/v/tree/master/thirdparty/"; // edit to accept stdlib url

impl Execute for Install {
    async fn execute(&self) -> Result<()> {
        if let (Some(url), Some(name)) = (&self.url, &self.package_name) {
            println!("Installing module {} from URL: {}", name, url);
            install_module_from_url(name, url)?;
            generate_docs(name).await?;
        } else if let Some(arg) = &self.url.as_ref().or(self.package_name.as_ref()) {
            if Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap().is_match(arg) {
                let url = arg.to_string();
                println!("Installing repository from URL: {}", url);
                install_repo_from_url(&url)?;
            } else {
                let name = arg.to_string();
                println!("Installing module {} from standard library", name);
                install_module_from_url(&name, STD_LIB_URL)?;
                fetch_docs_from_stdlib(&name).await?;
            }
        }

        Ok(())
    }
}

fn install_module_from_url(name: &String, url: &str) -> Result<()> {
    let repo_path = PathBuf::from(LOCATION).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default()
    );
    println!("Repo path: {}", repo_path.display());
    install_repo_from_url(url)?;
    Command::new("cd").args([repo_path.to_str().unwrap_or_default()]).status()?;
    println!("Current directory: {:?}", std::env::current_dir().unwrap_or_else(|_| PathBuf::from("Unknown")));
    let git_ls_files = Command::new("git")
        .arg("ls-files")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git ls-files");
    let grep = Command::new("grep")
        .arg(name)
        .stdin(git_ls_files.stdout.unwrap()) // Use the output of the previous command as input
        .stdout(Stdio::inherit()) // Print the output to the console
        .spawn()
        .expect("Failed to execute grep");
    let output = grep.wait_with_output().expect("Failed to wait for grep command");
    let file_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("Found module: {}", file_path);

    Ok(())
}

fn install_repo_from_url(url: &str) -> Result<()> {
    let repo_path = PathBuf::from(LOCATION).join(
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default()
    );

    dbg!(url.split('/').last().unwrap_or_default());
    clone_repo(url, repo_path.to_str().unwrap_or_default());

    Ok(())
}

fn clone_repo(url: &str, repo_path: &str) -> Result<()> {
    Command::new("git")
    .args(["clone", "--depth", "1", "--single-branch", "--jobs", "4", url, repo_path])
    .status()
    .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}

async fn fetch_docs_from_stdlib(package_name: &str) -> Result<()> {
    let url = format!("{}/{}", STD_LIB_URL, package_name);
    Ok(())
}

async fn generate_docs(package_name: &str) -> Result<()> {
    let package_path = Path::new(LOCATION).join(package_name);
    if !package_path.exists() {
        bail!("Package '{}' not found in {}", package_name, LOCATION);
    }

    let readme_path = package_path.join("README.md");
    let content = if readme_path.exists() {
        fs::read_to_string(readme_path)?
    } else {
        // If README.md doesn't exist, try to find a Verilog file
        let verilog_files: Vec<_> = fs::read_dir(&package_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "v" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if verilog_files.is_empty() {
            bail!("No README.md or Verilog files found in package '{}'", package_name);
        }

        fs::read_to_string(&verilog_files[0])?
    };
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let openai_client = ChatGPT::new(key)?;
    let response = openai_client.send_message(content).await?;
    let content = response.message().content.clone();

    // Save the generated documentation to a README.md file
    let readme_path = package_path.join("README.md");
    fs::write(&readme_path, content)?;

    println!("Documentation saved to: {}", readme_path.display());

    Ok(())
}