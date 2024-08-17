use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    pub version: u8,
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub submodules: Option<HashSet<String>>,
}

pub fn write_lockfile(lockfile: &Lockfile) -> Result<()> {
    let toml_string = toml::to_string(lockfile)?;
    std::fs::write("vpm.lock", toml_string)?;
    Ok(())
}
