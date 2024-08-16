mod cmd;
mod docs;
mod install;
mod uninstall;
mod dotf;
mod list;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    fn execute(&self) -> Result<()>;
}

impl Execute for Cmd {
    fn execute(&self) -> Result<()> {
        match self {
            Cmd::Install(cmd) => cmd.execute(),
            Cmd::Uninstall(cmd) => cmd.execute(),
            Cmd::Docs(cmd) => cmd.execute(),
            Cmd::Dotf(cmd) => cmd.execute(),
            Cmd::List(cmd) => cmd.execute(),
        }
    }
}
