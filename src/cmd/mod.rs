mod cmd;
mod include;
mod update;
mod remove;
mod dotf;
mod list;
mod install;
mod sim;
mod docs;
mod synth;
mod run;
mod build;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    async fn execute(&self) -> Result<()>;
}


impl Execute for Cmd {
    async fn execute(&self) -> Result<()> {
        match self {
            Cmd::Include(cmd) => cmd.execute().await,
            Cmd::Update(cmd) => cmd.execute().await,
            Cmd::Remove(cmd) => cmd.execute().await,
            Cmd::Dotf(cmd) => cmd.execute().await,
            Cmd::Install(cmd) => cmd.execute().await,
            Cmd::List(cmd) => cmd.execute().await,
            Cmd::Sim(cmd) => cmd.execute().await,
            Cmd::Docs(cmd) => cmd.execute().await,
            Cmd::Synth(cmd) => cmd.execute().await,
            Cmd::Build(cmd) => cmd.execute().await,
            Cmd::Run(cmd) => cmd.execute().await,
        }
    }
}
