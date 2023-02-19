use anyhow::Result;
use twilight_interactions::command::CreateCommand;
use twilight_model::{application::command::Command, gateway::payload::incoming::Ready};

use crate::{command::ping::Ping, Context};

impl Context {
    pub async fn ready(&self, ready: Ready) -> Result<()> {
        let commands: Vec<Command> = vec![Ping::create_command().into()];

        self.bot
            .http
            .interaction(ready.application.id)
            .set_global_commands(&commands)
            .await?;

        tracing::info!(
            "Ready as {}#{}",
            ready.user.name,
            ready.user.discriminator()
        );

        Ok(())
    }
}
