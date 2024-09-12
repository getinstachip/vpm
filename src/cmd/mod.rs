mod cmd;
mod upgrade;
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
mod load;
mod config;

use anyhow::Result;

pub use crate::cmd::cmd::*;

use crate::config_man::send_event;

pub trait Execute {
    async fn execute(&self) -> Result<()>;
}


impl Execute for Cmd {
    async fn execute(&self) -> Result<()> {
        match self {
            Cmd::Upgrade(cmd) => {
                cmd.execute().await?;
                send_event("upgrade")?;
                Ok(())
            },
            Cmd::Include(cmd) => {
                cmd.execute().await?;
                send_event("include")?;
                Ok(())
            },
            Cmd::Update(cmd) => {
                cmd.execute().await?;
                send_event("update")?;
                Ok(())
            },
            Cmd::Remove(cmd) => {
                cmd.execute().await?;
                send_event("remove")?;
                Ok(())
            },
            Cmd::Dotf(cmd) => {
                cmd.execute().await?;
                send_event("dotf")?;
                Ok(())
            },
            Cmd::Install(cmd) => {
                cmd.execute().await?;
                send_event("install")?;
                Ok(())
            },
            Cmd::List(cmd) => {
                cmd.execute().await?;
                send_event("list")?;
                Ok(())
            },
            Cmd::Sim(cmd) => {
                cmd.execute().await?;
                send_event("sim")?;
                Ok(())
            },
            Cmd::Docs(cmd) => {
                cmd.execute().await?;
                send_event("docs")?;
                Ok(())
            },
            Cmd::Synth(cmd) => {
                cmd.execute().await?;
                send_event("synth")?;
                Ok(())
            },
            Cmd::Load(cmd) => {
                cmd.execute().await?;
                send_event("load")?;
                Ok(())
            },
            Cmd::Run(cmd) => {
                cmd.execute().await?;
                send_event("run")?;
                Ok(())
            },
            Cmd::Config(cmd) => {
                cmd.execute().await?;
                send_event("config")?;
                Ok(())
            },
        }
    }
}
