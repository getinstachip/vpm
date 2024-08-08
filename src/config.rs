use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    /// The configured `Workspace` root path
    pub workspace_root: PathBuf,
    /// The current working directory
    pub cwd: PathBuf,
    /// VPM's home directory
    pub home: Option<PathBuf>,
}

impl Config {
    // TODO: Create workspace
    // #[must_use]
    // pub fn workspace(&self) -> Workspace {
    //     Workspace::new(&self.workspace_root, self)
    // }
}
