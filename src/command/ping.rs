use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_gateway::Latency;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::event::interaction_create::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(name = "ping", desc = "Responds with pong!")]
pub struct Ping;

impl InteractionContext<'_> {
    pub async fn execute_ping(self, latency: Latency) -> Result<()> {
        let now = std::time::Instant::now();

        self.handle
            .reply(Reply::new().content("calculating... :ping_pong:"))
            .await?;

        let latency = match latency.average() {
            Some(latency) => latency,
            None => {
                self.handle
                    .reply(
                        Reply::new().update_last().embed(
                            EmbedBuilder::new()
                                .title("Pong! :ping_pong:")
                                .description(format!(
                                    ":hourglass: Time: {} ms\n:stopwatch: WS: N/A\n\nWS is not available yet.",
                                    now.elapsed().as_millis()
                                )).color(0x00ff00)
                                .build(),
                        ),
                    )
                    .await?;
                return Ok(());
            }
        };

        self.handle
            .reply(
                Reply::new().update_last().embed(
                    EmbedBuilder::new()
                        .title("Pong! :ping_pong:")
                        .description(format!(
                            ":hourglass: Time: {} ms\n:stopwatch: WS: {} ms",
                            now.elapsed().as_millis(),
                            latency.as_millis()
                        ))
                        .color(0x00ff00)
                        .build(),
                ),
            )
            .await?;

        Ok(())
    }
}
