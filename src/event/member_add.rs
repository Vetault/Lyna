use anyhow::Result;
use twilight_mention::Mention;
use twilight_model::{
    gateway::payload::incoming::MemberAdd,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::Context;

impl Context {
    pub async fn member_add(&self, data: MemberAdd) -> Result<()> {
        self.welcome(&data).await?;
        Ok(())
    }
    async fn welcome(&self, data: &MemberAdd) -> Result<()> {
        let settings = sqlx::query!(
            "SELECT * FROM welcome WHERE guild_id = $1",
            data.guild_id.get() as i64
        )
        .fetch_one(&self.pool)
        .await?;

        match settings.channel_id {
            Some(channel_id) => {
                self.bot
                    .http
                    .create_message(Id::<ChannelMarker>::new(channel_id as u64))
                    .embeds(&[EmbedBuilder::new()
                        .color(0x00ff00)
                        .title("Wilkommen auf Vetault!")
                        .description(
                            &settings
                                .message
                                .unwrap_or("Welcome!".to_string())
                                .replace(
                                    "{@user}",
                                    data.member.user.id.mention().to_string().as_str(),
                                )
                                .replace("{user}", data.member.user.name.as_str()),
                        )
                        .build()])?
                    .await?;
            }
            None => {}
        }
        Ok(())
    }
}
