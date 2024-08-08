use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Install(Install),
    Uninstall(Uninstall),
    Include(Include),
}

/// Install a package
#[derive(Debug, Parser)]
#[clap(author)]
pub struct Install {
    #[arg(long)]
    pub url: Option<String>,
    #[arg(help="Name of package to install from OpenChips")]
    pub package_name: Option<String>,
}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Uninstall {
    #[arg(help="Name of package to Uninstall")]
    pub package_name: String,
}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Include {
    #[arg(help="Name of .v module to Include")]
    pub module_name: String,
    #[arg(help="Package to include from")]
    pub package_name: String,
}
