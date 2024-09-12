use anyhow::Result;
use directories::ProjectDirs;
use posthog_rs::{client, Event};
use std::fs;
use std::path::PathBuf;
use toml_edit::{DocumentMut, Item, Value};
use uuid::Uuid;

pub fn send_event(command: &str) -> Result<()> {
    let uuid = get_uuid()?;
    let client = client(std::env::var("POSTHOG_API_KEY").unwrap().as_str());
    let mut event = Event::new("user_action", &uuid);
    if let Err(_) = event.insert_prop("command", command) {}
    client.capture(event)?;
    Ok(())
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Instachip", "vpm")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .map(|mut path| {
            path.push("config.toml");
            path
        })
}

fn create_config() -> Result<()> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        fs::write(config_path.clone(), "").expect("Failed to create config.toml");
        let contents = fs::read_to_string(config_path.clone())?;
        let mut config_doc = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");
        config_doc["uuid"] = Item::Value(Value::from(Uuid::now_v7().to_string()));
        config_doc["os"] = Item::Value(Value::from(std::env::consts::OS));
        config_doc["arch"] = Item::Value(Value::from(std::env::consts::ARCH));
        fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    }
    Ok(())
}

fn get_uuid() -> Result<String> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let contents = fs::read_to_string(config_path)?;
    let config = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");
    Ok(config["uuid"].as_str().unwrap().to_string())
}