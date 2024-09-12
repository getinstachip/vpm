use crate::cmd::{Execute, Config};
use crate::config_man::set_analytics;
use anyhow::Result;

impl Execute for Config {
    async fn execute(&self) -> Result<()> {
        if self.analytics.is_some() {
            set_analytics(self.analytics.unwrap())?;
            println!("Analytics set to: {}", self.analytics.unwrap());
        }
        Ok(())
    }
}