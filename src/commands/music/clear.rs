use std::sync::Arc;

use songbird::tracks::TrackError;
use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, MessageBuilder},
};

#[command]
#[description = "Clear the song queue"]
pub struct Clear;

pub async fn clear(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
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

        if call.queue().is_empty() {
            let builder = MessageBuilder::new().error("No song is currently playing!");
            return command.create_message(&ctx, builder).await;
        }

        info!("Clearing song queue...");
        let result = call.queue().modify_queue(|q| {
            for item in q.into_iter().skip(1) {
                item.stop()?;
            }
            q.truncate(1);
            Ok::<_, TrackError>(())
        });

        match result {
            Ok(_) => {
                let builder = MessageBuilder::new().embed("Cleared song queue!");
                let _ = command.create_message(&ctx, builder).await;
            }
            Err(e) => {
                let builder =
                    MessageBuilder::new().error("Failed to clear the whole queue! Blame Joshi :c");
                let _ = command.create_message(&ctx, builder).await;
                return Err(e.into());
            }
        }
    }

    Ok(())
}
