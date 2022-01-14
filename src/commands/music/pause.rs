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
    info!("Pausing music...");
    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap()) {
        let call = call.lock().await;

        let handle = match call.queue().current() {
            Some(handle) => handle,
            None => {
                let builder = MessageBuilder::new().error("No song is currently playing!");
                return command.create_message(&ctx, builder).await;
            }
        };

        let state = handle.get_info().await?;

        let paused = state.playing == PlayMode::Pause;

        if !paused {
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
