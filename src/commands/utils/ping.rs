use std::{sync::Arc, time::Instant};

use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::ApplicationCommand,
};

use crate::{context::Context, error::BotResult};

#[command]
#[description = "Play ping pong with the bot"]
pub struct Ping;

async fn ping(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let curr_time = Instant::now();

    let response = InteractionResponse::ChannelMessageWithSource(CallbackData {
        allowed_mentions: None,
        components: None,
        content: Some(":ping_pong: Pong!".to_string()),
        embeds: vec![],
        flags: None,
        tts: None,
    });

    ctx.http
        .interaction_callback(command.id, &command.token, &response)
        .exec()
        .await?;

    let content = format!(":ping_pong: Pong! ({:?})", curr_time.elapsed());

    ctx.http
        .update_interaction_original(&command.token)?
        .content(Some(&content))?
        .exec()
        .await?;

    Ok(())
}
