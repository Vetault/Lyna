use anyhow::Result;
use regex::Regex;
use sparkle_convenience::{error::IntoError, reply::Reply};
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::{application_command::CommandData, InteractionData},
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{event::interaction_create::InteractionContext, translations::Lang};

use super::InteractionLang;

#[derive(CommandModel, CreateCommand)]
#[command(name = "debug", desc = "To Debug The Bot")]
pub struct Debug {
    #[command(desc = "To Debug The Bot")]
    pub debug: String,
}

impl InteractionContext<'_> {
    pub async fn execute_debug(&self) -> Result<()> {
        let client = &self.context.bot.http;
        let data = self.interaction.data.clone();
        let Some(InteractionData::ApplicationCommand(data)) = data else {return Ok(())};
        let data: CommandData = *data;
        let data = Debug::from_interaction(data.clone().into())?;

        match data.debug.as_str() {
            "welcome" => {
                let settings = sqlx::query!(
                    "SELECT * FROM welcome WHERE guild_id = $1",
                    self.interaction.guild_id.ok()?.get() as i64
                )
                .fetch_one(&self.context.pool)
                .await?;

                let channel = settings.channel_id.map(|c| c as u64).unwrap();
                tracing::error!("REEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
                tracing::info!("{}", channel);
                let channel = Id::<ChannelMarker>::new(channel);

                let message = settings.message;

                self.handle
                    .reply(Reply::new().content(format!(
                        "Welcome Channel is: {}\nWelcome Message is: {}",
                        channel.mention(),
                        message.map(|m| m).unwrap_or("None".to_string())
                    )))
                    .await?;
            }
            "language" => {
                self.handle
                    .reply(
                        Reply::new().embed(
                            EmbedBuilder::new()
                                .title("Language Debug")
                                .description("all Languages of the Bot")
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_bg: bg | Bulgarian | български",
                                        format!("{}", Lang::Bg.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_cz: cs | Czech | Čeština",
                                        format!("{}", Lang::Cs.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_dk: da | Danish | Dansk",
                                        format!("{}", Lang::Da.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_de: de | German | Deutsch",
                                        format!("{}", Lang::De.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_gr: el | Greek | Ελληνικά",
                                        format!("{}", Lang::El.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_us: :flag_gb: en-(US, GB) | English-(US, GB) | English-(US, GB)",
                                        format!("{}", Lang::En.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_es: es-ES | Spanish | Español",
                                        format!("{}", Lang::Es.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_fi: fi | Finnish | Suomi",
                                        format!("{}", Lang::Fi.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_fr: fr | French | Français",
                                        format!("{}", Lang::Fr.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_hu: hu | Hungarian | Magyar",
                                        format!("{}", Lang::Hu.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_id: id | Indonesian | Bahasa Indonesia",
                                        format!("{}", Lang::Id.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_it: it | Italian | Italiano",
                                        format!("{}", Lang::It.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_jp: ja | Japanese | 日本語",
                                        format!("{}", Lang::Ja.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_kr: ko | Korean | 한국어",
                                        format!("{}", Lang::Ko.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_nl: nl | Dutch | Nederlands",
                                        format!("{}", Lang::Nl.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_no: no | Norwegian | Norsk",
                                        format!("{}", Lang::No.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_pl: pl | Polish | Polski",
                                        format!("{}", Lang::Pl.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_ro: ro | Romanian, Romania | Română",
                                        format!("{}", Lang::Ro.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_ru: ru | Russian | Pусский",
                                        format!("{}", Lang::Ru.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_se: sv-SE | Swedish | Svenska",
                                        format!("{}", Lang::Sv.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_tr: tr | Turkish | Türkçe",
                                        format!("{}", Lang::Tr.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_ua: uk | Ukrainian | Українська",
                                        format!("{}", Lang::Uk.debug_language()),
                                    )
                                    .build(),
                                )
                                .field(
                                    EmbedFieldBuilder::new(
                                        ":flag_cn: zh-CN | Chinese, China | 中文",
                                        format!("{}", Lang::Zh.debug_language()),
                                    )
                                    .build(),
                                )
                                .color(0x008080)
                                .build(),
                        ),
                    )
                    .await?;
            }
            _ => {
                self.handle
                    .reply(Reply::new().content("Invalid Debug Command"))
                    .await?;

                return Ok(());
            }
        }

        /*         //create a regex to find "{user}" and {@user} and replace it with the user mention
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
            .await?; */

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
