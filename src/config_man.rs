use anyhow::Result;
use directories::ProjectDirs;
use rand::RngCore;
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use toml_edit::{DocumentMut, Item, Value, Table};
use uuid::Uuid;

use ring::aead::{self, Aad, LessSafeKey, Nonce};
use base64::{Engine as _, engine::general_purpose};
use ring::aead::UnboundKey;
use sha2::{Digest, Sha256};
use sys_info;

const POSTHOG_API_KEY: Option<&str> = option_env!("POSTHOG_API_KEY");
const DOCS_KEY: Option<&str> = option_env!("DOCS_KEY");

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
    user_table.insert("uuid", Item::Value(Value::from(create_uuid()?)));
    user_table.insert("os", Item::Value(Value::from(std::env::consts::OS)));
    user_table.insert("arch", Item::Value(Value::from(std::env::consts::ARCH)));

    config_doc.insert("tool", Item::Table(Table::new()));
    let tool_table = config_doc["tool"].as_table_mut().unwrap();
    tool_table.insert("version", Item::Value(Value::from(env!("CARGO_PKG_VERSION"))));

    config_doc.insert("options", Item::Table(Table::new()));
    let options_table = config_doc["options"].as_table_mut().unwrap();
    options_table.insert("analytics", Item::Value(Value::from(true)));

    config_doc.insert("metrics", Item::Table(Table::new()));
    let metrics_table = config_doc["metrics"].as_table_mut().unwrap();
    metrics_table.insert("docs_count", Item::Value(Value::from(0)));
    encrypt_docs_count(0)?;

    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}

fn create_uuid() -> Result<String> {
    let uuid = Uuid::now_v7().to_string();
    let os = sys_info::os_type().unwrap_or_default();
    let release = sys_info::os_release().unwrap_or_default();
    let arch = std::env::consts::ARCH.to_string();
    let cpu_num = sys_info::cpu_num().unwrap_or_default().to_string();
    let cpu_speed = sys_info::cpu_speed().unwrap_or_default().to_string();
    let mem_total = sys_info::mem_info().unwrap_or(sys_info::MemInfo { total: 0, free: 0, buffers: 0, cached: 0, swap_total: 0, swap_free: 0, avail: 0 }).total.to_string();
    let hostname = sys_info::hostname().unwrap_or_default();
    let timezone = std::env::var("TZ").unwrap_or_else(|_| "Unknown".to_string());

    let mut hasher = Sha256::new();
    hasher.update(uuid);
    hasher.update(os);
    hasher.update(release);
    hasher.update(arch);
    hasher.update(cpu_num);
    hasher.update(cpu_speed);
    hasher.update(mem_total);
    hasher.update(hostname);
    hasher.update(timezone);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
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

pub fn decrypt_docs_count() -> Result<u8> {
    let config_path = get_config_path().ok_or(anyhow::anyhow!("Failed to get config path"))?;
    if !config_path.exists() {
        create_config()?;
    }

    let config = fs::read_to_string(config_path)?;
    let config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    let encrypted_docs_base64 = config_doc["metrics"]["docs_count"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("docs_count not found in config"))?;
    let encrypted_docs = general_purpose::STANDARD
        .decode(encrypted_docs_base64)
        .map_err(|e| anyhow::anyhow!("Failed to decode docs count: {}", e))?;
    // Get the key from environment variable
    let docs_key_str = DOCS_KEY.ok_or_else(|| anyhow::anyhow!("DOCS_KEY is not set"))?;
    let key_bytes = hex::decode(docs_key_str).map_err(|e| anyhow::anyhow!("Invalid DOCS_KEY format: {}", e))?;

    // Create an AEAD key
    let unbound_key =
        UnboundKey::new(&aead::AES_256_GCM, &key_bytes).map_err(|_| anyhow::anyhow!("Invalid key"))?;
    let key = LessSafeKey::new(unbound_key);

    // Extract nonce and ciphertext
    if encrypted_docs.len() < 12 {
        return Err(anyhow::anyhow!("Ciphertext too short"));
    }
    let (nonce_bytes, ciphertext_and_tag) = encrypted_docs.split_at(12);
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes).map_err(|_| anyhow::anyhow!("Invalid nonce"))?;

    // Prepare mutable buffer for decryption
    let mut in_out = ciphertext_and_tag.to_vec();

    // Decrypt the data
    key.open_in_place(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| anyhow::anyhow!("Decryption failed"))?;

    // Convert decrypted data to string
    let decrypted_str =
        std::str::from_utf8(&in_out).map_err(|_| anyhow::anyhow!("Invalid UTF-8 in decrypted data"))?;
    let docs_count: u8 = decrypted_str.parse().map_err(|_| anyhow::anyhow!("Failed to parse decrypted data"))?;

    Ok(docs_count)
}

pub fn encrypt_docs_count(docs_count: u8) -> Result<()> {
    // Convert the docs_count to a string and then to bytes
    let docs_count_bytes = docs_count.to_string().into_bytes();
    // Get the key from the environment variable
    let docs_key_str = DOCS_KEY.ok_or_else(|| anyhow::anyhow!("DOCS_KEY is not set"))?;
    let key_bytes = hex::decode(docs_key_str).map_err(|e| anyhow::anyhow!("Invalid DOCS_KEY format: {}", e))?;

    // Create an AEAD key
    let unbound_key =
        UnboundKey::new(&aead::AES_256_GCM, &key_bytes).map_err(|_| anyhow::anyhow!("Invalid key"))?;
    let key = LessSafeKey::new(unbound_key);

    // Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    // Prepare buffer for encryption (data + space for the tag)
    let mut in_out = docs_count_bytes;
    in_out.extend_from_slice(&[0u8; 16]);

    // Encrypt the data
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| anyhow::anyhow!("Encryption failed"))?;

    // Prepend nonce to the ciphertext
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&in_out);

    // Encode the encrypted data to base64
    let encrypted_base64 = general_purpose::STANDARD.encode(encrypted_data);

    let config_path = get_config_path().unwrap();
    let config = fs::read_to_string(config_path.clone())?;
    let mut config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    config_doc["metrics"]["docs_count"] = Item::Value(Value::from(encrypted_base64));
    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");

    Ok(())
}