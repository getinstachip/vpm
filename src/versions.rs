use semver::{BuildMetadata, Comparator, Prerelease, Version};
use anyhow::Result;

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

    pub fn parse_semantic_version(raw_version: &str) -> Result<()> {
        let mut version =
            VersionReq::parse(raw_version)?;
        Ok(version.comparators.remove(0))
    }
}
