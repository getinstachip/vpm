mod cmd;
mod include;
mod update;
mod remove;
mod dotf;
mod list;
mod install;
mod sim;

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
            Cmd::Remove(cmd) => cmd.execute(),
            Cmd::Dotf(cmd) => cmd.execute(),
            Cmd::Install(cmd) => cmd.execute(),
            Cmd::List(cmd) => cmd.execute(),
            Cmd::Sim(cmd) => cmd.execute(),
        }
    }
}
