use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, MessageBuilder},
};

#[command]
#[description = "Stop the currently playing song and clear the queue"]
pub struct Stop;

pub async fn stop(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let author_id = command.user_id()?;
    let guild_id = command.guild_id.expect("Missing Guild ID for play command");

    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap()) {
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

        info!("Clearing song queue and stopping current song...");
        call.queue().stop();

        let builder = MessageBuilder::new().embed("Stopped playing music!");
        return command.create_message(&ctx, builder).await;
    }

    Ok(())
}
