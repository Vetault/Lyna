use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::{
        application_command::{CommandData, InteractionChannel},
        InteractionData,
    },
    channel::ChannelType,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::event::interaction_create::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(name = "settings", desc = "Setting Up The Bot")]
pub enum Settings {
    #[command(name = "set")]
    Set(Set),
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "set", desc = "Setting Up The Bot")]
pub enum Set {
    #[command(name = "welcome_channel")]
    WelcomeChannel(WelcomeChannel),
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "welcome_channel", desc = "Set up the welcome channel")]
pub struct WelcomeChannel {
    #[command(
        desc = "the channel to set as the welcome channel",
        channel_types = "guild_text"
    )]
    channel: Id<ChannelMarker>,
}

impl InteractionContext<'_> {
    pub async fn execute_settings(&self) -> Result<()> {
        let data = self.interaction.data.clone();
        tracing::info!("{}", self.interaction.guild_id.unwrap().to_string());

        let Some(InteractionData::ApplicationCommand(data)) = data else {return Ok(())};
        let data: CommandData = *data;

        let settings = Settings::from_interaction(data.clone().into())?;

        match settings {
            Settings::Set(set) => match set {
                Set::WelcomeChannel(welcome_channel) => {
                    sqlx::query("INSERT INTO settings (guild_id, name, value) VALUES ($1, $2, $3)")
                        .bind(
                            self.interaction
                                .guild_id
                                .unwrap()
                                .to_string()
                                .parse::<i64>()?,
                        )
                        .bind("welcome_channel")
                        .bind(welcome_channel.channel.to_string())
                        .execute(&self.context.pool)
                        .await?;

                    self.handle
                        .reply(Reply::new().content(format!(
                            "Welcome Channel: {}",
                            welcome_channel.channel.mention()
                        )))
                        .await?;
                }
            },
        }

        Ok(())
    }
}
