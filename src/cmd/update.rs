use anyhow::Result;

use crate::cmd::{Execute, Update};
use crate::cmd::include::get_head_commit_hash;
use crate::toml::{get_repo_links, add_top_module, remove_top_module};
use imara_diff::intern::InternedInput;
use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};

impl Execute for Update {
    async fn execute(&self) -> Result<()> {
        let module_path = &self.module_path;
        println!("Updating module '{}'", module_path);
        update_module(module_path, self.commit.as_deref())
    }
}

fn update_module(module_path: &str, commit: Option<&str>) -> Result<()> {
    let repo_links = get_repo_links(module_path);
    if repo_links.is_empty() {
        return Err(anyhow::anyhow!("No repositories found for module '{}'", module_path));
    }

    let chosen_repo = if repo_links.len() == 1 {
        repo_links.into_iter().next().unwrap()
    } else {
        println!("Multiple repositories found for module '{}'. Please choose one:", module_path);
        for (index, link) in repo_links.iter().enumerate() {
            println!("{}. {}", index + 1, link);
        }
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let index: usize = choice.trim().parse()?;
        repo_links.into_iter().nth(index - 1)
            .ok_or_else(|| anyhow::anyhow!("Invalid choice"))?
    };

    let head_commit_hash = get_head_commit_hash(&chosen_repo).unwrap();
    let commit_hash = commit.unwrap_or(&head_commit_hash);

    println!("Updating module '{}' to commit '{}'", module_path, commit_hash);
    let old_contents = std::fs::read_to_string(module_path)?;
    remove_top_module(&chosen_repo, module_path)?;
    add_top_module(&chosen_repo, module_path, commit_hash)?;
    let new_contents = std::fs::read_to_string(module_path)?;
    println!("Module '{}' updated to commit '{}'", module_path, commit_hash);

    display_diff(&old_contents, &new_contents);

    Ok(())
}

fn display_diff(old_contents: &str, new_contents: &str) {
    let input = InternedInput::new(old_contents, new_contents);
    let diff_output = diff(
        Algorithm::Histogram,
        &input,
        UnifiedDiffBuilder::new(&input)
    );

    println!("Diff:\n{}", diff_output);
}