use async_trait::async_trait;
use std::env::Args;

#[async_trait]
pub trait CommandHandler {
    fn parse(&mut self, args: &mut Args) -> ();
    async fn execute(&self) -> ();
}
