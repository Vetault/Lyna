use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::event::interaction_create::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(name = "test", desc = "test")]
pub struct Test2;

impl InteractionContext<'_> {
    pub async fn execute_test(&self) -> Result<()> {
        tracing::info!("test");

        let channel = sqlx::query!(
            "SELECT value FROM settings WHERE guild_id = $1 and name = $2",
            self.interaction
                .guild_id
                .unwrap()
                .to_string()
                .parse::<i64>()?,
            "welcome_channel"
        )
        .fetch_one(&self.context.pool)
        .await?;
        tracing::info!("test2");
        let channel = channel.value.parse::<Id<ChannelMarker>>()?;
        tracing::info!("test3");
        self.handle
            .reply(Reply::new().content(format!("Welcome Channel is: {}", channel.mention())))
            .await?;

        Ok(())
    }
}
