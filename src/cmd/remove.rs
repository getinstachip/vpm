use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

use anyhow::{Result, anyhow};

use crate::cmd::{Execute, Remove};
use crate::toml::{remove_top_module, get_repo_links};

impl Execute for Remove {
    async fn execute(&self) -> Result<()> {
        remove_module(&self.package_path)?;
        Ok(())
    }
}

fn remove_module(module_path: &str) -> Result<()> {
    let module_path = PathBuf::from(module_path);
    if !module_path.exists() {
        return Err(anyhow!("Module not found: {}", module_path.display()));
    }

    let module_name = module_path.file_name().unwrap().to_str().unwrap();
    
    // Ask for y/n confirmation
    print!("Are you sure you want to remove the module {}? (y/n): ", module_name);
    io::stdout().flush()?;
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    if confirmation.trim().to_lowercase() != "y" {
        return Ok(());
    }

    let repo_links = get_repo_links(module_name);

    let repo_link = match repo_links.len() {
        0 => return Err(anyhow!("No repository links found for module: {}", module_name)),
        1 => repo_links.into_iter().next().unwrap(),
        _ => {
            println!("Multiple repository links found for module: {}. Please choose the correct repository link.", module_name);
            for (i, link) in repo_links.iter().enumerate() {
                println!("{}. {}", i + 1, link);
            }
            
            let mut choice = String::new();
            print!("Enter your choice (1-{}): ", repo_links.len());
            io::stdout().flush()?;
            io::stdin().read_line(&mut choice)?;
            let index: usize = choice.trim().parse().map_err(|_| anyhow!("Invalid choice"))?;
            
            if index < 1 || index > repo_links.len() {
                return Err(anyhow!("Invalid choice"));
            }
            repo_links.iter().nth(index - 1)
                .ok_or_else(|| anyhow!("Invalid choice"))?
                .to_string()
        }
    };

    // Ask to enter the name of the module to confirm
    print!("To confirm removal, please re-type \"{}\" (without the quotes): ", module_name);
    io::stdout().flush()?;
    let mut confirmation_name = String::new();
    io::stdin().read_line(&mut confirmation_name)?;
    if confirmation_name.trim() != module_name {
        return Err(anyhow!("Module name does not match. Removal cancelled."));
    }

    fs::remove_file(&module_path)?;
    // Remove the corresponding header file if it exists
    let header_path = module_path.with_extension("vh");
    if header_path.exists() {
        fs::remove_file(&header_path)?;
        println!("Removed header file: {}", header_path.display());
    }
    remove_top_module(&repo_link, module_name)?;    
    println!("Removed module: {}", module_path.display());

    Ok(())
}
