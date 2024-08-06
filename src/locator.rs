use async_trait::async_trait;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;
use std::fs;
use std::path::Path;
use toml;

use crate::{
    embedding::{create_client, generate_embedding, embed_github_repository, vector_search, embed_library, create_index},
    errors::CommandError,
    command_handler::CommandHandler,
    errors::ParseError,
};

#[derive(Debug, Default)]
pub struct Locator {
    query: String,
    repo: bool,
    collection: bool,
    place_to_look: String,
}

impl Locator {
    pub fn new(query: String, place_to_look: String, repo: bool, collection: bool) -> Self {
        let query = query.trim_matches('"').to_string();
        
        Self {
            query,
            repo,
            collection,
            place_to_look,
        }
    }

    
}

#[async_trait]
impl CommandHandler for Locator {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();
        let query_embedding = generate_embedding(&self.query).await.unwrap();

        if self.repo {
            // embed the query
            let mut split = self.place_to_look.split('/');
            let package_author = split
                .next()
                .expect("Provided package author is empty")
                .to_string();
    
            let package_name = split
                .next()
                .expect("Provided package name is empty")
                .to_string();

            let index_name = format!("{}_{}", package_author, package_name).to_lowercase();
            let es_client = create_client().unwrap();
            // Create a progress bar for embedding
            // Check if the index already exists
            let index_exists = es_client.indices().exists(elasticsearch::indices::IndicesExistsParts::Index(&[&index_name])).send().await.unwrap().status_code().is_success();

            if !index_exists {
                let pb = ProgressBar::new_spinner();
                pb.set_style(ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap());
                pb.set_message("Embedding repository...");

                // embed the repo, store in elasticsearch, and return index
                embed_github_repository(&package_author, &package_name, &index_name).await.unwrap();

                // Finish the progress bar
                pb.finish_with_message("Repository embedded successfully!");
            }

            if self.collection {
                // Read the collection TOML file
                let collection_path_string = format!("./{}.Collections.toml", self.place_to_look);
                let collection_path = Path::new(&collection_path_string);
                let collection_content = fs::read_to_string(collection_path)
                    .map_err(CommandError::IOError)?;

                let collection_toml: toml::Value = toml::from_str(&collection_content)
                    .map_err(|e| CommandError::ParseError(format!("Failed to parse collection TOML: {}", e)))?;

                let index_name = self.place_to_look.replace(|c: char| !c.is_alphanumeric(), "_").to_lowercase();

                // Check if the index already exists
                let index_exists = es_client.indices().exists(elasticsearch::indices::IndicesExistsParts::Index(&[&index_name])).send().await
                    .map_err(|e| CommandError::ElasticsearchConnectionError(format!("Failed to check index existence: {}", e)))?
                    .status_code().is_success();

                if !index_exists {
                    // Create the index if it doesn't exist
                    create_index(&es_client, &index_name).await
                        .map_err(|e| CommandError::ElasticsearchConnectionError(format!("Failed to create index: {}", e)))?;
                    println!("Created new index: {}", index_name);
                } else {
                    println!("Using existing index: {}", index_name);
                }

                if let Some(packages) = collection_toml.get("packages") {
                    if let Some(packages_table) = packages.as_table() {
                        for (_, filepath) in packages_table {
                            if let Some(filepath_str) = filepath.as_str() {
                                let pb = ProgressBar::new_spinner();
                                pb.set_style(ProgressStyle::default_spinner()
                                    .template("{spinner:.green} {msg}")
                                    .unwrap());
                                pb.set_message(format!("Embedding package: {}", filepath_str));

                                // Embed the package
                                embed_library(Path::new(filepath_str), &index_name).await
                                    .map_err(|e| CommandError::EmbeddingError(e.to_string()))?;

                                pb.finish_with_message(format!("Package embedded successfully: {}", filepath_str));
                            }
                        }
                    }
                }
            }
                // search the index
            let results = vector_search(&es_client, &index_name, query_embedding, 10).await.unwrap();

            // Print all results (only the name)
            println!("{} relevant modules found for '{}':", results.len(), self.query);
            for (_i, result) in results.iter().enumerate() {
                println!("{}", result.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
            }

            if results.is_empty() {
                println!("No results found for the given query.");
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }

    async fn list() -> Result<(), ParseError> {
        Ok(())
    }
}