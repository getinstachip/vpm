use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Install(Install),
    Uninstall(Uninstall),
    Init(Init),
    Docs(Docs),
}

/// Install a package
#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    disable_version_flag = true,
    propagate_version = true,
    version
)]
pub struct Install {
    #[arg(help = "Name of package to install from OpenChips")]
    pub package_name: Option<String>,
    #[arg(help = "URL of repository to install from")]
    pub url: Option<String>,
    #[arg(help="Version of package to install")]
    pub version: Option<String>,
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
    disable_version_flag = true,
    propagate_version = true,
    version
)]
pub struct Init {
    #[arg(help = "Name of project to initialize")]
    pub project_name: String,
    #[arg(help = "SemVer version of project")]
    pub version: Option<String>,
    #[arg(help = "Description of project")]
    pub description: Option<String>,
    #[arg(help = "Authors of project (comma + space separated)")]
    pub authors: Option<String>,
    #[arg(help = "License of project (<license>: <location>, comma + space separated)")]
    pub license: Option<String>,
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
    pub package_name: Option<String>,
    #[arg(help = "URL of repository to fetch documentation for")]
    pub url: Option<String>,
}