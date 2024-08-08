mod cmd;
mod install;
mod uninstall;

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
        }
    }
}
