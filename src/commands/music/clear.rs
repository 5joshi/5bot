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
    info!("Clearing song queue...");
    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap()) {
        let call = call.lock().await;

        if call.queue().is_empty() {
            let builder = MessageBuilder::new().error("No song is currently playing!");
            return command.create_message(&ctx, builder).await;
        }

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
