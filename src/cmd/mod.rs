mod cmd;
mod install;
mod uninstall;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    async fn execute(&self) -> Result<()>;
}

impl Execute for Cmd {
    async fn execute(&self) -> Result<()> {
        match self {
            Cmd::Install(cmd) => cmd.execute().await,
            Cmd::Uninstall(cmd) => cmd.execute().await,
        }
    }
}
