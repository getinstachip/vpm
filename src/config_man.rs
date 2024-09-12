use anyhow::Result;
use directories::ProjectDirs;
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use toml_edit::{DocumentMut, Item, Value, Table};
use uuid::Uuid;

const POSTHOG_API_KEY: Option<&str> = option_env!("POSTHOG_API_KEY");

pub async fn send_event(command: String) -> Result<()> {
    if get_analytics()? {
        let uuid = get_uuid()?;
        let version = env!("CARGO_PKG_VERSION").to_string();
        let api_key = POSTHOG_API_KEY.expect("POSTHOG_API_KEY environment variable not set").to_string();
        
        let client = Client::new();
        let payload = json!({
            "api_key": api_key,
            "event": "user_action",
            "distinct_id": uuid,
            "properties": {
                "command": command,
                "version": version
            }
        });

        let _response = client.post("https://us.i.posthog.com/capture/")
            .json(&payload)
            .send()
            .await?;

        // if !response.status().is_success() {
        //     eprintln!("Failed to send event to PostHog: {}", response.status());
        // }
    }
    Ok(())
}

pub fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Instachip", "vpm")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .map(|mut path| {
            path.push("config.toml");
            path
        })
}

pub fn create_config() -> Result<()> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(&config_path)?;
    }
    fs::write(config_path.clone(), "").expect("Failed to create config.toml");
    let contents = fs::read_to_string(config_path.clone())?;
    let mut config_doc = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");

    config_doc.insert("user", Item::Table(Table::new()));
    let user_table = config_doc["user"].as_table_mut().unwrap();
    user_table.insert("uuid", Item::Value(Value::from(Uuid::now_v7().to_string())));
    user_table.insert("os", Item::Value(Value::from(std::env::consts::OS)));
    user_table.insert("arch", Item::Value(Value::from(std::env::consts::ARCH)));

    config_doc.insert("tool", Item::Table(Table::new()));
    let tool_table = config_doc["tool"].as_table_mut().unwrap();
    tool_table.insert("version", Item::Value(Value::from(env!("CARGO_PKG_VERSION"))));

    config_doc.insert("options", Item::Table(Table::new()));
    let options_table = config_doc["options"].as_table_mut().unwrap();
    options_table.insert("analytics", Item::Value(Value::from(true)));

    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}

fn get_uuid() -> Result<String> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let contents = fs::read_to_string(config_path)?;
    let config = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");
    Ok(config["user"]["uuid"].as_str().unwrap().to_string())
}

pub fn set_analytics(value: bool) -> Result<()> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let config = fs::read_to_string(config_path.clone())?;
    let mut config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    config_doc["options"]["analytics"] = Item::Value(Value::from(value));
    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}

fn get_analytics() -> Result<bool> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let config = fs::read_to_string(config_path.clone())?;
    let config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    Ok(config_doc["options"]["analytics"].as_bool().unwrap())
}

pub fn set_version(version: &str) -> Result<()> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let config = fs::read_to_string(config_path.clone())?;
    let mut config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    config_doc["tool"]["version"] = Item::Value(Value::from(version));
    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}   