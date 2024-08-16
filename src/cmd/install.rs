use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::{Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;

use toml_edit::{DocumentMut, Item, value, Array};

use crate::cmd::{Execute, Install};

const STD_LIB_URL: &str = "https://github.com/getinstachip/openchips";

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(https?://|git://|ftp://|file://|www\.)[\w\-\.]+\.\w+(/[\w\-\.]*)*/?$").unwrap()
});

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        fs::create_dir_all("./vpm_modules")?;
        match (&self.url, &self.package_name) {
            (Some(url), Some(name)) => {
                println!("Installing module '{}' from URL: '{}'", name, url);
                install_module_from_url(name, url)
            }
            (Some(url), None) | (None, Some(url)) if URL_REGEX.is_match(url) => {
                println!("Installing repository from URL: '{}'", url);
                install_repo_from_url(url, "./vpm_modules/")?;
                add_dependency(None, None, Some(url))
            }
            (None, Some(name)) => {
                println!("Installing module '{}' from standard library", name);
                install_module_from_url(name, STD_LIB_URL)
            }
            _ => {
                println!("Command not found!");
                Ok(())
            }
        }
    }
}

fn name_from_url(url: &str) -> &str {
    url.rsplit('/').find(|&s| !s.is_empty()).unwrap_or_default()
}

pub fn install_module_from_url(module: &str, url: &str) -> Result<()> {
    let package_name = name_from_url(url);
    let tmp_path = PathBuf::from("/tmp").join(package_name);

    install_repo_from_url(url, "/tmp/")?;

    fs::create_dir_all(format!("./vpm_modules/{}", module))?;
    let module_list = process_module(package_name, module, module, &mut HashSet::new())?;
    add_dependency(Some(package_name), Some(module_list), Some(url))?;

    fs::remove_dir_all(tmp_path)?;

    Ok(())
}

fn process_module(package_name: &str, top_module: &str, module: &str, visited: &mut HashSet<String>) -> Result<HashSet<String>> {
    let module_name = module.strip_suffix(".v").unwrap_or(module);
    if !visited.insert(module_name.to_string()) {
        return Ok(HashSet::new());
    }
    println!("Processing module '{}'", module_name);
    let tmp_path = PathBuf::from("/tmp").join(package_name);
    for entry in WalkDir::new(&tmp_path).into_iter().filter_map(Result::ok) {
        if entry.file_name() == module {
            let target_path = PathBuf::from("./vpm_modules").join(format!("{}/{}", top_module, module));

            fs::copy(
                entry.path(),
                target_path.with_extension("v"),
            )?;

            let contents = fs::read_to_string(entry.path())?;
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_verilog::language())?;

            let tree = parser.parse(&contents, None).context("Failed to parse file")?;
            let root_node = tree.root_node();

            let header_content = generate_headers(root_node, &contents)?;
            fs::write(
                target_path.with_extension("vh"),
                header_content,
            )?;

            for submodule in get_submodules(root_node, &contents)? {
                if !visited.contains(&submodule) {
                    process_module(package_name, top_module, &format!("{}.v", submodule), visited)?;
                }
            }

            break;
        }
    }

    Ok(visited.clone())
}

fn generate_headers(root_node: Node, contents: &str) -> Result<String> {
    static QUERY: Lazy<Query> = Lazy::new(|| {
        Query::new(
            tree_sitter_verilog::language(),
            "(module_declaration
                (module_header
                    (module_keyword)
                    (simple_identifier) @module_name)
                (module_nonansi_header
                    (parameter_port_list)? @params
                    (list_of_ports) @ports)
            )
            (module_declaration
                (module_header
                    (module_keyword)
                    (simple_identifier) @module_name)
                (module_ansi_header
                    (parameter_port_list)? @params
                    (list_of_port_declarations)? @ports)
            )",
        )
        .expect("Failed to create query")
    });

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&QUERY, root_node, contents.as_bytes());

    let mut header_content = String::new();

    for match_ in matches {
        let mut module_name = "";
        let mut params = "";
        let mut ports = "";

        for capture in match_.captures {
            let capture_text = &contents[capture.node.byte_range()];
            match capture.index {
                0 => module_name = capture_text,
                1 => params = capture_text,
                2 => ports = capture_text,
                _ => {}
            }
        }
        
        header_content.push_str(&format!(
            "module {} {}(\n{}\n{});\n\n// TODO: Add module implementation\n\nendmodule // {}\n\n",
            module_name,
            if params.is_empty() { "" } else { "#(\n" },
            params,
            ports,
            module_name
        ));
    }

    Ok(header_content)
}

fn get_submodules(root_node: Node, contents: &str) -> Result<HashSet<String>> {
    static QUERY: Lazy<Query> = Lazy::new(|| {
        Query::new(
            tree_sitter_verilog::language(),
            "(module_or_generate_item 
                (module_instantiation 
                    (simple_identifier) @module_submodule
                )
            )
            (module_or_generate_item 
                (udp_instantiation 
                    (simple_identifier) @module_submodule
                )
            )",
        )
        .expect("Failed to create query")
    });

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&QUERY, root_node, contents.as_bytes());

    let mut submodules = HashSet::new();

    for match_ in matches {
        for capture in match_.captures {
            if capture.index == 0 {
                let capture_text = &contents[capture.node.byte_range()];
                submodules.insert(capture_text.to_string());
            }
        }
    }

    Ok(submodules)
}

fn install_repo_from_url(url: &str, location: &str) -> Result<()> {
    let repo_path = Path::new(location).join(name_from_url(url));

    Command::new("git")
        .args([ "clone", "--depth", "1", "--single-branch", "--jobs", "4",
            url, repo_path.to_str().unwrap_or_default(),
        ])
        .status()
        .with_context(|| format!("Failed to clone repository from URL: '{}'", url))?;

    Ok(())
}

fn add_dependency(package_name: Option<&str>, modules: Option<HashSet<String>>, url: Option<&str>) -> Result<()> {
    if !PathBuf::from("./vpm.toml").exists() {
        fs::write("./vpm.toml", initial_vpm_toml())?;
    }
    let contents = fs::read_to_string("./vpm.toml")?;
    let mut document: DocumentMut = contents.parse()?;

    let dependencies = document["dependencies"].or_insert(Item::Table(toml_edit::Table::new())).as_table_mut().unwrap();

    if let Some(package_name) = package_name {
        let package = dependencies.entry(package_name).or_insert(Item::Table(toml_edit::Table::new())).as_table_mut().unwrap();

        if let Some(url) = url {
            package.insert("git", value(url));
        } else {
            package.insert("version", value("v0.1.0"));
        }

        if let Some(modules) = modules {
            let mut modules_array = Array::new();
            for module in modules {
                modules_array.push(module);
            }
            package.insert("modules", value(modules_array));
        }
    }

    fs::write("./vpm.toml", document.to_string())?;
    Ok(())
}

fn initial_vpm_toml() -> String {
    "[package]\n\
    name = \"my-vpm-package\"\n\
    version = \"0.1.0\"\n\
    authors = [\"author-name <author-email>\"]\n\
    description = \"My VPM package\"\n\
    license = \"My License\"\n\
    repository = \"https://github.com/<username>/<repository>\"\n\
    ".to_string()
}
