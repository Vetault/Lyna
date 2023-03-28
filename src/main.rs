use dotenvy::dotenv;
use futures_util::StreamExt;
use sea_query::{Iden, PostgresQueryBuilder, Query};
use sparkle_convenience::Bot;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};
use std::{env, sync::Arc};
use translations::Lang;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream::ShardEventStream, EventTypeFlags, Intents};
use twilight_http::Client;

pub mod translations {
    rosetta_i18n::include_translations!();
}

mod command;
mod event;
mod handler;
mod utils;

#[derive(Debug)]
pub struct Context {
    pub bot: Bot,
    pub cache: Arc<InMemoryCache>,
    pub pool: PgPool,
    pub tz: chrono_tz::Tz,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord Token in the environment");

    let intents = Intents::GUILD_MESSAGES
        | Intents::GUILD_VOICE_STATES
        | Intents::GUILDS
        | Intents::GUILD_MEMBERS;
    let event_types = EventTypeFlags::READY
        | EventTypeFlags::INTERACTION_CREATE
        | EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::MEMBER_ADD;
    let cache = Arc::new(InMemoryCache::new());

    let (bot, mut shards) = Bot::new(token.clone(), intents, event_types).await?;

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;

    tracing::info!("{}", Lang::Sv.welcome_module_description());

    let context = Arc::new(Context {
        cache,
        bot,
        pool,
        tz: env::var("TZ")
            .expect("Expected a timezone in the environment")
            .parse()
            .expect("failed to parse timezone"),
    });

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

        tokio::spawn(Context::handle_event(
            event,
            context.clone(),
            shard.latency().clone(),
        ));
    }

    Ok(())
}

#[derive(Iden)]
enum Users {
    Table,
    #[iden = "id"]
    Id,
}
