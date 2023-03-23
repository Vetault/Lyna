use anyhow::Result;
use twilight_interactions::command::CreateCommand;
use twilight_model::{application::command::Command, gateway::payload::incoming::Ready};

use crate::{
    command::{debug::Debug, ping::Ping, settings::Settings, test::Test2},
    Context,
};

impl Context {
    pub async fn ready(&self, ready: Ready) -> Result<()> {
        let commands: Vec<Command> = vec![
            Ping::create_command().into(),
            Settings::create_command().into(),
            Test2::create_command().into(),
            Debug::create_command().into(),
        ];

        tracing::info!("{:#?}", Settings::create_command());

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
