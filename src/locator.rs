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
    package_author: String,
    package_name: String,
}

impl Locator {
    pub fn new(query: String, place_to_look: String) -> Self {
        let query = query.trim_matches('"').to_string();

        let mut split = place_to_look.split('/');
        let package_author = split
            .next()
            .expect("Provided package author is empty")
            .to_string();

        let package_name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();
        
        Self {
            query,
            package_author,
            package_name,
        }
    }

    
}

#[async_trait]
impl CommandHandler for Locator {
    async fn execute(&self) -> Result<(), CommandError> {
        let now = Instant::now();
        let query_embedding = generate_embedding(&self.query).await.unwrap();

        // embed the query


        let index_name = format!("{}_{}", self.package_author, self.package_name).to_lowercase();
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
            embed_github_repository(&self.package_author, &self.package_name, &index_name).await.unwrap();

            // Finish the progress bar
            pb.finish_with_message("Repository embedded successfully!");
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

        let elapsed = now.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
        Ok(())
    }

    async fn list() -> Result<(), ParseError> {
        Ok(())
    }
}