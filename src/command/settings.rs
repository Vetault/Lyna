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
    command::InteractionGetUser, event::interaction_create::InteractionContext, translations::Lang,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "settings",
    desc = "Setting Up The Bot",
    dm_permission = false,
    default_permissions = "permissions"
)]
pub enum Settings {
    #[command(name = "set")]
    Set(Set),
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "set",
    desc = "Setting Up The Bot",
    desc_localizations = "translate"
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
}

impl InteractionContext<'_> {
    pub async fn execute_settings(&self) -> Result<()> {
        let data = self.interaction.data.clone();
        tracing::info!("{}", self.interaction.guild_id.unwrap().to_string());

        let Some(InteractionData::ApplicationCommand(data)) = data else {return Ok(())};
        let data: CommandData = *data;
        let settings = Settings::from_interaction(data.clone().into())?;

        match settings {
            Settings::Set(set) => {
                match set {
                    Set::Welcome(welcome) => {
                        let settings = sqlx::query!(
                            "INSERT INTO welcome (guild_id, channel_id, message)
                            VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO
                            UPDATE
                            SET channel_id = coalesce($2, welcome.channel_id),
                                message = coalesce($3, welcome.message) RETURNING *",
                            self.interaction.guild_id.ok()?.get() as i64,
                            welcome.channel.map(|i| i.get() as i64),
                            welcome.message
                        )
                        .fetch_one(&self.context.pool)
                        .await?;

                        tracing::info!("{:#?}", &self.interaction.get_user().locale);

                        let lang = self.interaction.locale.as_ref().ok()?;
                        tracing::info!("{:#?}", lang);

                        //transform the user language into a Lang enum
                        let lang = match lang.as_str() {
                            "bg" => Lang::Bg,
                            "cs" => Lang::Cs,
                            "da" => Lang::Da,
                            "de" => Lang::De,
                            "el" => Lang::El,
                            "en-US" => Lang::En,
                            "en-GB" => Lang::En,
                            "es-ES" => Lang::Es,
                            "fi" => Lang::Fi,
                            "fr" => Lang::Fr,
                            "hu" => Lang::Hu,
                            "id" => Lang::Id,
                            "it" => Lang::It,
                            "ja" => Lang::Ja,
                            "ko" => Lang::Ko,
                            "nl" => Lang::Nl,
                            "no" => Lang::No,
                            "pl" => Lang::Pl,
                            "ro" => Lang::Ro,
                            "ru" => Lang::Ru,
                            "sv-SE" => Lang::Sv,
                            "tr" => Lang::Tr,
                            "uk" => Lang::Uk,
                            "zh-CN" => Lang::Zh,
                            _ => Lang::En,
                        };

                        self.handle
                            .reply(
                                Reply::new().embed(
                                    EmbedBuilder::new()
                                        .title("Welcome Module")
                                        .description(format!(
                                            "**Successfully set Welcome Settings**\n\nChannel: \n\nTest: {}",
                                            //settings.channel_id.ok()? as u64
                                            lang.welcome_module_description()
                                        ))
                                        .build(),
                                ),
                            )
                            .await?;
                    }
                }
            }
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
