use anyhow::Result;
use regex::Regex;
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
        let client = &self.context.bot.http;

        //create a regex to find "{user}" and {@user} and replace it with the user mention
        let text = "{user}, {@user} {channel} {@channel} {dontexist}";
        let rep = |caps: &regex::Captures| {
            let cap = caps.get(1).unwrap().as_str();
            match cap {
                "@user" => self
                    .interaction
                    .member
                    .as_ref()
                    .unwrap()
                    .user
                    .as_ref()
                    .unwrap()
                    .mention()
                    .to_string(),
                "user" => self
                    .interaction
                    .member
                    .as_ref()
                    .unwrap()
                    .user
                    .as_ref()
                    .unwrap()
                    .name
                    .to_string(),
                "@channel" => self
                    .interaction
                    .channel_id
                    .as_ref()
                    .unwrap()
                    .mention()
                    .to_string(),
                "channel" => self.interaction.channel_id.as_ref().unwrap().to_string(),
                _ => cap.to_string(),
            }
        };

        let re = Regex::new(r"\{(@?\w+)\}")?;
        let text = re.replace_all(text, rep);

        self.handle
            .reply(Reply::new().content(format!("{}", text)))
            .await?;

        /*         let channel = sqlx::query!(
            "SELECT value FROM settings WHERE guild_id = $1 and name = $2",
            self.interaction
                .guild_id
                .unwrap()
                .to_string()
                .parse::<i64>()?,
            "welcome_channel"
        )
        .fetch_one(&self.context.pool)
        .await?; */
        /*         let channel = channel.value.parse::<Id<ChannelMarker>>()?;
        self.handle
            .reply(Reply::new().content(format!("Welcome Channel is: {}", channel.mention())))
            .await?; */

        //client.create_message(channel).content("test")?.await?;

        Ok(())
    }
}
