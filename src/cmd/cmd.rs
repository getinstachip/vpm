use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Include(Include),
    Update(Update),
    Uninstall(Uninstall),
    Docs(Docs),
    Dotf(Dotf),
    Install(Install),
    List(List),
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
pub struct Include {
    #[arg(help = "Name of package to include")]
    pub package_name: Option<String>,
    #[arg(help = "URL of repository to include from")]
    pub url: Option<String>,
    #[arg(help="Version of package to include")]
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
pub struct Update {
    #[arg(help = "Name of package to update")]
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
pub struct Docs{
    #[arg(help = "Name of module to generate documentation for")]
    pub module: Option<String>,
    #[arg(help = "Url of repository to generate documentation for")]
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
pub struct Dotf {
    #[arg(help = "Path to top module to generate filelist for")]
    pub path_to_top_module: String,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]

pub struct Install {
    #[arg(help = "Name of tool to install")]
    pub tool_name: String,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub struct List {}