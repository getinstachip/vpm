use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VersionData {
    pub name: String,
    pub version: String,
    pub dist: Dist,
}

#[derive(Debug, Deserialize)]
pub struct Dist {
    pub tarball: String,
}

#[derive(Deserialize)]
pub struct PackageData {
    pub versions: HashMap<String, VersionData>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageLock {
    #[serde(rename="isLatest")]
    pub is_latest: bool,
}

impl PackageLock {
    pub fn new(is_latest: bool) -> Self {
        Self {
            is_latest
        }
    }
}
