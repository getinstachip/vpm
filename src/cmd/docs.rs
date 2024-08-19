// use anyhow::{Result, Context};
// use reqwest::Client;
// use tokio::runtime::Runtime;
// use std::path::PathBuf;
// use serde_json::json;
// use std::fs;
// use indicatif::{ProgressBar, ProgressStyle};
// use crate::cmd::include::clone_repo;

// use crate::cmd::{Execute, Docs};

// impl Execute for Docs {
//     fn execute(&self) -> Result<()> {
//         let rt = Runtime::new()?;
//         if let Some(url) = &self.url {
//             let content = rt.block_on(fetch_module_content(&self.module, url))?;
//             rt.block_on(generate_docs(&self.module, &content, None))?;
//         } else {
//             let module_name = self.module.strip_suffix(".v").or_else(|| self.module.strip_suffix(".sv")).unwrap_or(&self.module);
//             let vpm_modules_dir = PathBuf::from("./vpm_modules");
            
//             match find_and_read_module(&vpm_modules_dir, module_name) {
//                 Some((content, module_path)) => {
//                     println!("Generating documentation for local module '{}'", module_name);
//                     rt.block_on(generate_docs(module_name, &content, Some(module_path)))?;
//                 },
//                 None => println!("Module '{}' not found in vpm_modules. Please provide a URL to a repository containing the module.", module_name),
//             }
//         }
//         Ok(())
//     }
// }

// async fn fetch_module_content(module: &str, url: &str) -> Result<String> {
//     let tmp_dir = tempfile::tempdir()?;
//     let repo_path = tmp_dir.path();

//     // Clone the repository using the public clone_repo function
//     clone_repo(url, repo_path)?;

//     let module_name = module.strip_suffix(".v").or_else(|| module.strip_suffix(".sv")).unwrap_or(module);

//     println!("Fetching content for module: {}", module_name);
    
//     // Search for the module file recursively
//     fn find_module_file(dir: &std::path::Path, module_name: &str) -> Option<std::path::PathBuf> {
//         if let Ok(entries) = fs::read_dir(dir) {
//             for entry in entries.filter_map(Result::ok) {
//                 let path = entry.path();
//                 if path.is_dir() {
//                     if let Some(found_path) = find_module_file(&path, module_name) {
//                         return Some(found_path);
//                     }
//                 } else {
//                     let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
//                     if file_name == module_name || 
//                        file_name == &format!("{}.v", module_name) || 
//                        file_name == &format!("{}.sv", module_name) {
//                         return Some(path);
//                     }
//                 }
//             }
//         }
//         None
//     }

//     let module_file = find_module_file(repo_path, module_name)
//         .context("Module file not found in repository")?;

//     // Read the module content
//     let content = fs::read_to_string(module_file)?;

//     Ok(content)
// }

// fn find_and_read_module(dir: &PathBuf, module_name: &str) -> Option<(String, PathBuf)> {
//     fn find_module_recursively(dir: &PathBuf, module_name: &str) -> Option<PathBuf> {
//         if let Ok(entries) = fs::read_dir(dir) {
//             for entry in entries.filter_map(Result::ok) {
//                 let path = entry.path();
//                 if path.is_dir() {
//                     if let Some(found_path) = find_module_recursively(&path, module_name) {
//                         return Some(found_path);
//                     }
//                 } else if path.file_stem().and_then(|s| s.to_str()) == Some(module_name) {
//                     return Some(path);
//                 }
//             }
//         }
//         None
//     }

//     find_module_recursively(dir, module_name)
//         .and_then(|path| fs::read_to_string(&path).ok().map(|content| (content, path)))
// }

// fn format_text(text: &str) -> String {
//     text.replace("\\n", "\n")
//         .replace("\\'", "'")
//         .replace("\\\"", "\"")
//         .replace("\\\\", "\\")
// }

// async fn generate_docs(module: &str, content: &str, module_path: Option<PathBuf>) -> Result<()> {
//     let pb = ProgressBar::new(100);
//     pb.set_style(ProgressStyle::default_bar()
//         .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
//         .unwrap()
//         .progress_chars("#>-"));
    
//     pb.set_position(33);
//     pb.set_message("Generating documentation...");

//     let client = Client::new();
//     let api_url = "https://bmniatl2bh.execute-api.us-east-1.amazonaws.com/dev/getApiKey";
//     let response = client.post(api_url)
//         .header("Content-Type", "application/json")
//         .json(&json!({ "code": content }))
//         .send().await?;

//     let documentation = format_text(&response.text().await?);

//     pb.set_position(66);
//     pb.set_message("Writing documentation to file...");

//     let module_name = module.strip_suffix(".v").or_else(|| module.strip_suffix(".sv")).unwrap_or(module);
//     let readme_path = if let Some(path) = module_path {
//         path.with_file_name(format!("{}_README.md", module_name))
//     } else {
//         let dir = format!("./vpm_modules/{}", module_name);
//         fs::create_dir_all(&dir)?;
//         PathBuf::from(&dir).join(format!("{}_README.md", module_name))
//     };
//     tokio::fs::write(&readme_path, documentation).await?;

//     pb.set_position(100);
//     pb.finish_with_message(format!("Documentation for {} written to {}", module, readme_path.display()));

//     Ok(())
// }
