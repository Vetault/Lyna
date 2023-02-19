use futures_util::StreamExt;
use sparkle_convenience::Bot;
use std::{env, sync::Arc};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream::ShardEventStream, EventTypeFlags, Intents};
use twilight_http::Client;

mod command;
mod event;
mod handler;
mod utils;

#[derive(Debug)]
pub struct Context {
    pub client: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
    pub bot: Bot,
    //pub shards: Vec<Shard>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord Token in the environment");

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let event_types =
        EventTypeFlags::READY | EventTypeFlags::INTERACTION_CREATE | EventTypeFlags::MESSAGE_CREATE;
    let client = Arc::new(Client::new(token.clone()));
    let cache = Arc::new(InMemoryCache::new());

    let (bot, mut shards) = Bot::new(token.clone(), intents, event_types).await?;

    let context = Arc::new(Context { client, cache, bot });
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };
        context.cache.update(&event);

        tokio::spawn(handler::handle_event(
            event,
            context.clone(),
            shard.latency().clone(),
        ));
    }

    Ok(())
}
