pub struct Package {
    /// Information to identify the `Package`.
    id: PackageId,
}

impl Package {
    /// Get a reference to the `Package`'s name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.id.name
    }

    /// Get a reference to the `Package`'s `Version`
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.id.version
    }
}
