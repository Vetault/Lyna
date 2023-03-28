use anyhow::Result;
use chrono::TimeZone;
use serde::Deserialize;
use serde_json::json;
use sparkle_convenience::{error::IntoError, reply::Reply};
use time::{
    macros::{date, datetime, time},
    Date, Time,
};
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::{
        application_command::{CommandData, InteractionChannel},
        InteractionData,
    },
    guild::Permissions,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    command::{InteractionGetUser, InteractionLang},
    event::interaction_create::InteractionContext,
    translations::Lang,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "settings", desc = "Setting Up The Bot", dm_permission = false)]
pub enum Settings {
    #[command(name = "set")]
    Set(Set),
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "set",
    desc = "Setting Up The Bot",
    desc_localizations = "translate",
    default_permissions = "permissions"
)]
pub enum Set {
    #[command(name = "welcome")]
    Welcome(Welcome),
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "welcome",
    desc = "Setting up the welcome module",
    desc_localizations = "translate"
)]
pub struct Welcome {
    #[command(
        desc = "set or update the channel as the welcome channel",
        channel_types = "guild_text"
    )]
    channel: Option<Id<ChannelMarker>>,
    #[command(desc = "set or update the welcome message")]
    message: Option<String>,
    #[command(desc = "Activate or deactivate the welcome module")]
    active: Option<bool>,
}

impl InteractionContext<'_> {
    pub async fn execute_settings(&self) -> Result<()> {
        let data = self.interaction.data.clone();

        let Some(InteractionData::ApplicationCommand(data)) = data else {return Ok(())};
        let data: CommandData = *data;
        let settings = Settings::from_interaction(data.clone().into())?;

        let lang = self.interaction.lang();

        match settings {
            Settings::Set(set) => match set {
                Set::Welcome(welcome) => {
                    let x = welcome.channel.map(|i| i.get() as i64);
                    tracing::info!("{:?}", x);
                    let settings = sqlx::query!(
                        "INSERT INTO welcome (guild_id, channel_id, message, active)
                            VALUES ($1, $2, $3, $4) ON CONFLICT (guild_id) DO
                            UPDATE
                            SET channel_id = coalesce($2, welcome.channel_id),
                                message = coalesce($3, welcome.message),
                                active = coalesce($4, welcome.active)
                                 RETURNING *",
                        self.interaction.guild_id.ok()?.get() as i64,
                        welcome.channel.map(|i| i.get() as i64),
                        welcome.message.map(|m| m.replace("\\n", "\n")),
                        welcome.active
                    )
                    .fetch_one(&self.context.pool)
                    .await?;

                    let message = &settings.message;

                    self.handle
                        .reply(
                            Reply::new().embed(
                                EmbedBuilder::new()
                                    .title(format!("{}", lang.welcome_module_success_title()))
                                    .description(
                                        lang.welcome_module_success_description(
                                            settings.active.unwrap_or(false).to_string(),
                                            settings
                                                .channel_id
                                                .map(|i| {
                                                    Id::<ChannelMarker>::new(i as u64)
                                                        .mention()
                                                        .to_string()
                                                })
                                                .unwrap_or("**Null**".to_string()),
                                            message
                                                .as_ref()
                                                .map(|m| m.replace("\n", "\\n"))
                                                .unwrap_or("**Null**".to_string()),
                                            message.as_ref().unwrap_or(&"**Null**".to_string()),
                                        ),
                                    )
                                    .build(),
                            ),
                        )
                        .await?;
                }
            },
        }

        Ok(())
    }
}

fn permissions() -> Permissions {
    Permissions::MANAGE_GUILD
}

//create a function to translate the description and return IntoIterator<Item = (ToString, ToString)>
fn translate() -> [(&'static str, &'static str); 2] {
    [
        ("en-US", Lang::En.welcome_module_description()),
        ("de", Lang::De.welcome_module_description()),
    ]
}
