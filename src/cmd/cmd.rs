use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    Install(Install),
}

// Install a package
#[derive(Debug, Parser)]
#[clap(author)]
pub struct Install {
    #[clap(num_args=1.., required=true)]
    pub package: String,
}
