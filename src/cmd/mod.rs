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
                send_event("upgrade".to_string()).await?;
                Ok(())
            },
            Cmd::Include(cmd) => {
                cmd.execute().await?;
                send_event("include".to_string()).await?;
                Ok(())
            },
            Cmd::Update(cmd) => {
                cmd.execute().await?;
                send_event("update".to_string()).await?;
                Ok(())
            },
            Cmd::Remove(cmd) => {
                cmd.execute().await?;
                send_event("remove".to_string()).await?;
                Ok(())
            },
            Cmd::Dotf(cmd) => {
                cmd.execute().await?;
                send_event("dotf".to_string()).await?;
                Ok(())
            },
            Cmd::Install(cmd) => {
                cmd.execute().await?;
                send_event("install".to_string()).await?;
                Ok(())
            },
            Cmd::List(cmd) => {
                cmd.execute().await?;
                send_event("list".to_string()).await?;
                Ok(())
            },
            Cmd::Sim(cmd) => {
                cmd.execute().await?;
                send_event("sim".to_string()).await?;
                Ok(())
            },
            Cmd::Docs(cmd) => {
                cmd.execute().await?;
                send_event("docs".to_string()).await?;
                Ok(())
            },
            Cmd::Synth(cmd) => {
                cmd.execute().await?;
                send_event("synth".to_string()).await?;
                Ok(())
            },
            Cmd::Load(cmd) => {
                cmd.execute().await?;
                send_event("load".to_string()).await?;
                Ok(())
            },
            Cmd::Run(cmd) => {
                cmd.execute().await?;
                send_event("run".to_string()).await?;
                Ok(())
            },
            Cmd::Config(cmd) => {
                cmd.execute().await?;
                send_event("config".to_string()).await?;
                Ok(())
            },
        }
    }
}
