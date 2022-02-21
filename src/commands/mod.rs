mod message;
mod music;
mod osu;
mod utils;

use std::sync::Arc;

use message::Activity;
use music::Clear;
use twilight_model::application::{command::Command, interaction::ApplicationCommand};
use utils::{Ping, Roll};

use crate::{
    commands::{
        message::{Complete, Impersonate},
        osu::Suijisim,
    },
    context::Context,
    error::{BotResult, Error},
    utils::ApplicationCommandExt,
};
pub use message::MessageActivity;

use self::music::{Pause, Play, Queue, Skip, Stop, Volume};

pub fn twilight_commands() -> Vec<Command> {
    vec![
        Clear::define(),
        Complete::define(),
        Impersonate::define(),
        Pause::define(),
        Ping::define(),
        Play::define(),
        Queue::define(),
        Skip::define(),
        Stop::define(),
        Suijisim::define(),
        Volume::define(),
        Roll::define(),
        Activity::define(),
    ]
}

fn log_slash(ctx: &Context, command: &ApplicationCommand, cmd_name: &str) {
    let username = command.username().unwrap_or("<unknown user>");
    let mut location = String::with_capacity(32);

    match command.guild_id.and_then(|id| ctx.cache.guild(id)) {
        Some(guild) => {
            location.push_str(guild.name.as_str());
            location.push(':');

            match ctx.cache.guild_channel(command.channel_id) {
                Some(channel) => location.push_str(channel.name()),
                None => location.push_str("<uncached channel>"),
            }
        }
        None => location.push_str("Private"),
    }

    info!("[{}] {}: /{}", location, username, cmd_name);
}

pub async fn handle_interaction(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let name = command.data.name.as_str();
    log_slash(&ctx, &command, name);
    ctx.stats.increment_slash_command(name);

    match name {
        Activity::NAME => Activity::run(ctx, command).await,
        Clear::NAME => Clear::run(ctx, command).await,
        Complete::NAME => Complete::run(ctx, command).await,
        Impersonate::NAME => Impersonate::run(ctx, command).await,
        Pause::NAME => Pause::run(ctx, command).await,
        Ping::NAME => Ping::run(ctx, command).await,
        Play::NAME => Play::run(ctx, command).await,
        Queue::NAME => Queue::run(ctx, command).await,
        Skip::NAME => Skip::run(ctx, command).await,
        Stop::NAME => Stop::run(ctx, command).await,
        Suijisim::NAME => Suijisim::run(ctx, command).await,
        Volume::NAME => Volume::run(ctx, command).await,
        Roll::NAME => Roll::run(ctx, command).await,
        _ => Err(Error::UnknownInteraction {
            command: Box::new(command),
        }),
    }
}
