use std::sync::Arc;

use songbird::tracks::PlayMode;
use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, MessageBuilder},
};

#[command]
#[description = "Pause or unpause the song that's currently playing"]
pub struct Pause;

pub async fn pause(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let author_id = command.user_id()?;
    let guild_id = command.guild_id.expect("Missing Guild ID for play command");

    if let Some(call) = ctx.songbird.get(guild_id) {
        let call = call.lock().await;
        let channel_opt = ctx
            .cache
            .voice_state(author_id, guild_id)
            .and_then(|state| state.channel_id);

        match (channel_opt, call.current_channel()) {
            (Some(id1), Some(id2)) if id1.0 != id2.0 => {
                let builder =
                    MessageBuilder::new().error("You aren't in the same voice channel as me!");
                return command.create_message(&ctx, builder).await;
            }
            (None, _) => {
                let builder = MessageBuilder::new().error("You aren't in a voice channel!");
                return command.create_message(&ctx, builder).await;
            }
            _ => {}
        };

        let handle = match call.queue().current() {
            Some(handle) => handle,
            None => {
                let builder = MessageBuilder::new().error("No song is currently playing!");
                return command.create_message(&ctx, builder).await;
            }
        };

        let state = handle.get_info().await?;

        let paused = state.playing == PlayMode::Pause;
        info!("{} music...", if paused { "Resuming" } else { "Pausing" });

        if paused {
            let result = call.queue().resume();

            if let Err(e) = result {
                let builder =
                    MessageBuilder::new().error("Failed to resume the song! Blame Joshi :c");
                let _ = command.create_message(&ctx, builder).await;
                return Err(e.into());
            }
        } else {
            let result = call.queue().pause();

            if let Err(e) = result {
                let builder =
                    MessageBuilder::new().error("Failed to pause the song! Blame Joshi :c");
                let _ = command.create_message(&ctx, builder).await;
                return Err(e.into());
            }
        }

        let content = format!(
            "{} the current song!",
            if paused { "Resumed" } else { "Paused" }
        );
        let builder = MessageBuilder::new().embed(content);
        let _ = command.create_message(&ctx, builder).await;
    }

    Ok(())
}
