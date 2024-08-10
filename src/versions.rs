use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Ok, Result};
use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};

pub const EMPTY_VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

pub const LATEST: &str = "latest";

type PackageDetails = (String, Option<Comparator>);

#[derive(Debug)]
pub struct Dist {
    pub tarball: String,
}

#[derive(Debug)]
pub struct VersionData {
    pub name: String,
    pub version: String,
    pub dist: Dist,
}

#[derive(Debug)]
pub struct Versions;
impl Versions {
    pub fn parse_raw_package_details(details: String) -> (String, String) {
        let mut split = details.split('@');

        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        match split.next() {
            Some(version_raw) => (name, version_raw.to_string()),
            None => (name, LATEST.to_string()),
        }
    }

    pub fn parse_semantic_version(raw_version: &str) -> Result<Comparator> {
        let mut version = VersionReq::parse(raw_version)?;
        Ok(version.comparators.remove(0))
    }

    pub fn parse_semantic_package_details(details: String) -> Result<PackageDetails> {
        let (name, version_raw) = Self::parse_raw_package_details(details);

        if version_raw == LATEST {
            return Ok((name, None));
        }

        let comparator = Self::parse_semantic_version(&version_raw)?;
        Ok((name, Some(comparator)))
    }

    pub fn resolve_full_version(semantic_version: Option<&Comparator>) -> Option<String> {
        let latest = LATEST.to_string();

        let semantic_version = match semantic_version {
            Some(semantic_version) => semantic_version,
            None => return Some(latest),
        };

        let (minor, patch) = match (semantic_version.minor, semantic_version.patch) {
            (Some(minor), Some(patch)) => (minor, patch),
            _ => return None,
        };

        match semantic_version.op {
            Op::Greater | Op::GreaterEq | Op::Wildcard => Some(latest),
            Op::Exact | Op::LessEq | Op::Tilde | Op::Caret => Some(Self::stringify_from_numbers(
                semantic_version.major,
                minor,
                patch,
            )),
            _ => None,
        }
    }

    pub fn resolve_partial_version(
        semantic_version: Option<&Comparator>,
        available_versions: &HashMap<String, VersionData>,
    ) -> Result<String> {
        let semantic_version = semantic_version
            .expect("Function should not be called as the version can be resolved to 'latest'");

        let mut versions = available_versions.iter().collect::<Vec<_>>();

        // Serde scrambles the order of the hashmap so we need to reorder it to find the latest
        Self::sort(&mut versions);

        if semantic_version.op == Op::Less {
            if let (Some(minor), Some(patch)) = (semantic_version.minor, semantic_version.patch) {
                let version_position = versions.iter().position(|(ver, _)| {
                    ver == &&Self::stringify_from_numbers(semantic_version.major, minor, patch)
                })
                .context("Invalid version provided")?;

                return Ok(versions
                    .get(version_position- 1)
                    .expect("Invalid version provided (no smaller versions available)")
                    .0
                    .to_string());
            }
        }

        for (version_str, _) in versions.iter().rev() {
            let version = Version::from_str(version_str.as_str()).unwrap_or(EMPTY_VERSION);

            if semantic_version.matches(&version) {
                return Ok(version_str.to_string());
            }
        }

        Err(anyhow!("Invalid version provided"))
    }

    pub fn stringify(name: &String, version: &String) -> String {
        format!("{}@{}", name, version)
    }

    pub fn is_latest(version_string: Option<&String>) -> bool {
        match version_string {
            Some(version) => version == LATEST,
            None => false,
        }
    }

    fn sort(versions_vec: &mut [(&String, &VersionData)]) {
        versions_vec.sort_by(|a, b| a.0.cmp(b.0))
    }

    pub fn stringify_from_numbers(major: u64, minor: u64, patch: u64) -> String {
        format!("{}.{}.{}", major, minor, patch)
    }
}
