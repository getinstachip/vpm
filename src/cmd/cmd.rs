use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Include(Include),
    Update(Update),
    Uninstall(Uninstall),
    Dotf(Dotf),
    Install(Install),
    List(List),
    Run(Run),
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
    #[arg(help = "URL of repository to include from")]
    pub url: String,
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
    #[arg(help = "Full module path of package to update")]
    pub package_path: String,
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
    #[arg(help = "Full module path of package to Uninstall")]
    pub package_path: String,
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
    #[arg(help = "Tool to install")]
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

pub struct Run {
    #[arg(long, help = "Simulation target (e.g., 'sim')")]
    pub target: String,

    #[arg(long, help = "EDA tool to use (e.g., 'verilator')")]
    pub tool: String,

    #[arg(help = "Name of the core to simulate")]
    pub core_name: String,

    #[arg(last = true, help = "Additional arguments to pass to FuseSoC")]
    pub additional_args: Vec<String>,
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