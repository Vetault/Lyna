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
    id::{marker::ChannelMarker, Id},
};

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
        desc = "set or update the channel as the welcome channel",
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
                    let settings = sqlx::query!(
                        "SELECT value FROM settings WHERE guild_id = $1 and name = $2",
                        self.interaction
                            .guild_id
                            .unwrap()
                            .to_string()
                            .parse::<i64>()?,
                        "welcome_channel"
                    )
                    .fetch_all(&self.context.pool)
                    .await?;

                    tracing::info!("{:?}", settings);

                    if settings.len() > 0 {
                        let new_channel = &sqlx::query!(
                            "UPDATE settings SET value = $1 WHERE guild_id = $2 and name = $3 RETURNING value",
                            welcome_channel.channel.to_string(),
                            self.interaction
                                .guild_id
                                .unwrap()
                                .to_string()
                                .parse::<i64>()?,
                            "welcome_channel"
                        )
                        .fetch_all(&self.context.pool)
                        .await?[0];

                        tracing::info!("{:#?}", new_channel);
                        self.handle
                            .reply(
                                Reply::new()
                                    .content(format!(
                                        "Welcome channel was updated to {}",
                                        new_channel.value.parse::<Id<ChannelMarker>>()?.mention()
                                    ))
                                    .ephemeral(),
                            )
                            .await?;
                        return Ok(());
                    }

                    sqlx::query("INSERT INTO settings (guild_id, name, value, type) VALUES ($1, $2, $3, $4)")
                        .bind(
                            self.interaction
                                .guild_id
                                .unwrap()
                                .to_string()
                                .parse::<i64>()?,
                        )
                        .bind("welcome_channel")
                        .bind(welcome_channel.channel.to_string()).bind("i64")
                        .execute(&self.context.pool)
                        .await?;

                    self.handle
                        .reply(
                            Reply::new()
                                .content(format!(
                                    "Welcome channel set to {}",
                                    welcome_channel.channel.mention()
                                ))
                                .ephemeral(),
                        )
                        .await?;
                }
            },
        }

        Ok(())
    }
}
