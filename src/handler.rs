/* use crate::events::interaction_create::InteractionCreateEvent;
use crate::events::ready::ReadyEvent;
use crate::Context; */
use anyhow::Result;
use std::sync::Arc;
use twilight_gateway::{stream::ShardRef, Event, Latency};

use crate::Context;

pub async fn handle_event(event: Event, context: Arc<Context>, latency: Latency) -> Result<()> {
    let event = event.clone();
    match event {
        Event::Ready(data) => {
            //ReadyEvent::execute(*data, context.clone()).await?
            context.ready(*data).await;
        }
        Event::InteractionCreate(interaction) => {
            context.interaction_create(interaction.0, latency).await;
        }
        _ => {}
    }
    Ok(())
}
