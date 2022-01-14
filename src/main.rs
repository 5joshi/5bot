macro_rules! unwind_error {
    ($log:ident, $err:ident, $($arg:tt)+) => {
        {
            $log!($($arg)+, $err);
            let mut err: &dyn ::std::error::Error = &$err;

            while let Some(source) = err.source() {
                $log!("  - caused by: {}", source);
                err = source;
            }
        }
    };
}

mod commands;
mod context;
mod database;
mod error;
mod logging;
mod osu_irc;
mod stats;
mod utils;

use context::Context;
use dashmap::DashSet;
use database::Database;
use error::{BotResult, Error};

use futures::StreamExt;
use osu_irc::IrcClient;
use parking_lot::RwLock;
use rosu_v2::Osu;
use songbird::Songbird;
use stats::BotStats;
use std::{collections::VecDeque, env, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::Events, Cluster, Event, EventTypeFlags, Intents};
use twilight_http::Client as HttpClient;
use twilight_model::{
    application::interaction::Interaction,
    gateway::presence::{ActivityType, Status},
    id::GuildId,
};
use twilight_standby::Standby;

use crate::{commands::handle_interaction, utils::SERVER_ID};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate slash_command_macro;

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");
    if let Err(e) = runtime.block_on(async_main()) {
        unwind_error!(error, e, "Critical Error in main: {}")
    };
}

async fn async_main() -> BotResult<()> {
    logging::initialize();
    dotenv::dotenv().ok();

    // Initialize the tracing subscriber.
    let token = env::var("DISCORD_TOKEN").expect("Missing environment variable (DISCORD_TOKEN).");

    let http = HttpClient::new(token.clone());
    let user_id = http.current_user().exec().await?.model().await?.id;
    http.set_application_id(user_id.0.into());

    let intents = Intents::GUILDS
        | Intents::GUILD_MEMBERS
        | Intents::GUILD_MESSAGES
        | Intents::GUILD_MESSAGE_REACTIONS
        | Intents::DIRECT_MESSAGES
        | Intents::DIRECT_MESSAGE_REACTIONS
        | Intents::GUILD_VOICE_STATES;

    let ignore_flags = EventTypeFlags::BAN_ADD
        | EventTypeFlags::BAN_REMOVE
        | EventTypeFlags::CHANNEL_PINS_UPDATE
        | EventTypeFlags::GIFT_CODE_UPDATE
        | EventTypeFlags::GUILD_INTEGRATIONS_UPDATE
        | EventTypeFlags::INTEGRATION_CREATE
        | EventTypeFlags::INTEGRATION_DELETE
        | EventTypeFlags::INTEGRATION_UPDATE
        | EventTypeFlags::INVITE_CREATE
        | EventTypeFlags::INVITE_DELETE
        | EventTypeFlags::PRESENCE_UPDATE
        | EventTypeFlags::PRESENCES_REPLACE
        | EventTypeFlags::SHARD_PAYLOAD
        | EventTypeFlags::STAGE_INSTANCE_CREATE
        | EventTypeFlags::STAGE_INSTANCE_DELETE
        | EventTypeFlags::STAGE_INSTANCE_UPDATE
        | EventTypeFlags::TYPING_START
        | EventTypeFlags::WEBHOOKS_UPDATE;

    let (cluster, events) = Cluster::builder(token, intents)
        .event_types(EventTypeFlags::all() - ignore_flags)
        .http_client(http.clone())
        .build()
        .await?;
    cluster.up().await;

    let songbird = Songbird::twilight(cluster.clone(), user_id);
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::CHANNEL | ResourceType::GUILD | ResourceType::VOICE_STATE)
        .build();

    let database_url =
        env::var("DATABASE_URL").expect("Missing environment variable (DATABASE_URL).");
    let database = Database::new(&database_url).await?;

    let client_id = env::var("OSU_CLIENT_ID")
        .expect("Missing environment variable (OSU_CLIENT_ID).")
        .parse::<u64>()
        .expect("osu! client ID must be a number.");
    let client_secret =
        env::var("OSU_CLIENT_SECRET").expect("Missing environment variable (OSU_CLIENT_SECRET).");

    let commands = commands::twilight_commands();

    http.set_guild_commands(GuildId(SERVER_ID), &commands)?
        .exec()
        .await?;

    http.set_global_commands(&[])?.exec().await?;

    let osu = Osu::new(client_id, client_secret).await?;

    // TODO: DashSet should contain list of users to track
    let irc = IrcClient::new(DashSet::new());

    // let trackdata = RwLock::new(None);

    let standby = Standby::new();

    let stats = BotStats::new(osu.metrics());

    let ctx = Context {
        cache,
        cluster,
        database,
        http,
        irc,
        osu,
        songbird,
        standby,
        stats,
    };

    let ctx = Arc::new(ctx);

    tokio::select! {
        _ = event_loop(Arc::clone(&ctx), events) => {}
        _ = wait_for_ctrl_c() => {}
    };

    info!("Shutting down cluster...");
    ctx.cluster.down();

    info!("Clearing song queue...");
    let guilds = ctx.cache.iter().guilds();
    if let Some(call) = ctx.songbird.get(SERVER_ID) {
        let call = call.lock().await;
        call.queue().stop();
    }

    Ok(())
}

async fn event_loop(ctx: Arc<Context>, mut events: Events) {
    while let Some((shard_id, event)) = events.next().await {
        ctx.cache.update(&event);
        ctx.songbird.process(&event).await;
        ctx.standby.process(&event);
        let ctx = Arc::clone(&ctx);

        tokio::spawn(async move {
            if let Err(why) = handle_event(ctx, event, shard_id).await {
                unwind_error!(error, why, "Error while handling event: {}");
            }
        });
    }
}

async fn wait_for_ctrl_c() {
    if let Err(why) = tokio::signal::ctrl_c().await {
        unwind_error!(error, why, "Failed to listen for ctrl-c event. {:?}");
    }
}

async fn handle_event(ctx: Arc<Context>, event: Event, shard_id: u64) -> BotResult<()> {
    match event {
        Event::GatewayInvalidateSession(reconnect) => {
            ctx.stats.event_counts.gateway_invalidate.inc();

            if reconnect {
                warn!(
                    "Gateway has invalidated session for shard {}, but its reconnectable",
                    shard_id
                );
            } else {
                warn!("Gateway has invalidated session for shard {}", shard_id);
            }
        }
        Event::GatewayReconnect => {
            info!("Gateway requested shard {} to reconnect", shard_id);
            ctx.stats.event_counts.gateway_reconnect.inc();
        }
        Event::InteractionCreate(e) => {
            if let Interaction::ApplicationCommand(command) = e.0 {
                handle_interaction(ctx, *command).await?;
            }
        }
        Event::Ready(_) => {
            info!("Shard {} is ready", shard_id);

            let fut =
                ctx.set_shard_activity(shard_id, Status::Online, ActivityType::Playing, "osu!");

            match fut.await {
                Ok(_) => info!("Game is set for shard {}", shard_id),
                Err(why) => unwind_error!(
                    error,
                    why,
                    "Failed to set shard activity at ready event for shard {}: {}",
                    shard_id
                ),
            }
        }
        Event::MessageCreate(e) => ctx.database.insert_message(&(*e).0).await.map(|_| ())?,
        Event::Resumed => info!("Shard {} is resumed", shard_id),
        Event::RoleCreate(_) => ctx.stats.event_counts.role_create.inc(),
        Event::RoleDelete(_) => ctx.stats.event_counts.role_delete.inc(),
        Event::RoleUpdate(_) => ctx.stats.event_counts.role_update.inc(),
        Event::ShardConnected(_) => info!("Shard {} is connected", shard_id),
        Event::ShardConnecting(_) => info!("Shard {} is connecting...", shard_id),
        Event::ShardDisconnected(_) => info!("Shard {} is disconnected", shard_id),
        Event::ShardIdentifying(_) => info!("Shard {} is identifying...", shard_id),
        Event::ShardReconnecting(_) => info!("Shard {} is reconnecting...", shard_id),
        Event::ShardResuming(_) => info!("Shard {} is resuming...", shard_id),
        _ => {}
    }
    Ok(())
}
