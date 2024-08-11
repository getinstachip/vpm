use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Install(Install),
    Uninstall(Uninstall),
    Docs(Docs),
}

/// Install a package
#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub struct Install {
    #[arg(help = "Name of package to install from OpenChips")]
    pub package_name: Option<String>,
    #[arg(help = "URL of repository to install from")]
    pub url: Option<String>,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub struct Uninstall {
    #[arg(help = "Name of package to Uninstall")]
    pub package_name: String,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub struct Docs {
    #[arg(help = "Name of package to fetch documentation for")]
    pub package_name: String,
    pub url: Option<String>,
}