use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

use self::{ping::Ping, settings::Settings, test::Test2};

pub mod ping;
pub mod settings;
pub mod test;

pub fn commands() -> Vec<Command> {
    vec![
        Ping::create_command().into(),
        Settings::create_command().into(),
        Test2::create_command().into(),
    ]
}
