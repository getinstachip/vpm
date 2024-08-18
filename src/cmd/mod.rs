mod cmd;
mod docs;
mod include;
mod update;
mod uninstall;
mod dotf;
mod list;
mod install;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    fn execute(&self) -> Result<()>;
}

impl Execute for Cmd {
    fn execute(&self) -> Result<()> {
        match self {
            Cmd::Include(cmd) => cmd.execute(),
            Cmd::Update(cmd) => cmd.execute(),
            Cmd::Uninstall(cmd) => cmd.execute(),
            Cmd::Docs(cmd) => cmd.execute(),
            Cmd::Dotf(cmd) => cmd.execute(),
            Cmd::Install(cmd) => cmd.execute(),
            Cmd::List(cmd) => cmd.execute(),
        }
    }
}
