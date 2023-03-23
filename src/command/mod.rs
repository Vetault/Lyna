use self::{ping::Ping, settings::Settings, test::Test2};
use crate::event::interaction_create::InteractionContext;
use crate::translations::Lang;
use sparkle_convenience::error::IntoError;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::{command::Command, interaction::InteractionData};
use twilight_model::{application::interaction::Interaction, user::User};

pub trait InteractionGetUser {
    fn get_user(&self) -> User;
}

pub trait InteractionLang {
    fn lang(&self) -> Lang;
}

impl InteractionGetUser for Interaction {
    fn get_user(&self) -> User {
        if let Some(member) = &self.member {
            return member.user.clone().unwrap();
        }

        self.user.clone().unwrap()
    }
}

impl InteractionLang for Interaction {
    fn lang(&self) -> Lang {
        match self.locale.as_ref() {
            Some(locale) => match locale.as_str() {
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
            },
            None => Lang::En,
        }
    }
}

pub mod debug;
pub mod ping;
pub mod settings;
pub mod test;

/* impl InteractionContext<'_> {
    fn get_data(&self) -> Option<Command> {
        self.interaction.data.clone().and_then(|data| {
            if let InteractionData::ApplicationCommand(data) = data {
                Some(*data)
            } else {
                None
            }
        })
    }
} */

pub fn commands() -> Vec<Command> {
    vec![
        Ping::create_command().into(),
        Settings::create_command().into(),
        Test2::create_command().into(),
    ]
}
