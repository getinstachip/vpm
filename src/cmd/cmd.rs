use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Include(Include),
    Update(Update),
    Remove(Remove),
    Dotf(Dotf),
    Docs(Docs),
    Install(Install),
    List(List),
    Sim(Sim),
    Synth(Synth),
    Load(Load),
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
    #[arg(long, short, help = "Include as a repository")]
    pub repo: bool,
    #[arg(help = "URL of repository to include from")]
    pub url: String,
    #[arg(long, help = "Include RISC-V specific modules")]
    pub riscv: bool,
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
    #[arg(help = "Full module path of module to update")]
    pub module_path: String,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub struct Remove {
    #[arg(help = "Full module path of package to remove")]
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
pub struct Docs{
    #[arg(help = "Path of module to generate documentation for")]
    pub module_path: String,
    #[arg(help = "Url of repository to generate documentation for")]
    pub url: Option<String>,
    #[arg(long, help = "Generate documentation in offline mode")]
    pub offline: bool,
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

pub struct Sim {
    #[arg(help = "List of Verilog files to simulate")]
    pub verilog_files: Vec<String>,
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

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]  
pub struct Synth {
    #[arg(help = "Top module path to synthesize")]
    pub top_module_path: String,
    #[arg(long, help = "Use RISC-V toolchain")]
    pub riscv: bool,
    #[arg(long, help = "Path to RISC-V core", required_if_eq("riscv", "true"))]
    pub core_path: Option<String>,
    #[arg(long, help = "Specify target board")]
    pub board: Option<String>,
    #[arg(long, help = "Generate synthesis script")]
    pub gen_yosys_script: bool,
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]

pub struct Load {
    #[arg(help = "Path to top module to load")]
    pub top_module_path: String,
    #[arg(help = "Path to constraints file")]
    pub constraints_path: String,
    #[arg(long, help = "Use RISC-V toolchain")]
    pub riscv: bool,

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
    #[arg(help = "Path to the program to run")]
    pub program_path: String,

    #[arg(long, help = "Use RISC-V toolchain")]
    pub riscv: bool,
}