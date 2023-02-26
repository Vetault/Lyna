use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::event::interaction_create::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(name = "asdas", desc = "asdasd")]
pub struct asdas;

impl InteractionContext<'_> {
    pub async fn execute_asdas(&self) -> Result<()> {
        let now = std::time::Instant::now();

        self.handle.reply(Reply::new().content("Response")).await?;

        Ok(())
    }
}
