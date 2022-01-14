use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, EmbedBuilder, MessageBuilder},
};

#[command]
#[description = "Display the current song queue"]
pub struct Queue;

pub async fn queue(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    info!("Displaying current song queue...");
    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap().get()) {
        let call = call.lock().await;

        let len_queue = call.queue().len();

        if len_queue < 2 {
            let builder = MessageBuilder::new()
                .embed("The queue is currently empty!\nAdd more songs by using /music play");
            return command.create_message(&ctx, builder).await;
        }

        let mut content = String::new();
        for (i, handle) in call.queue().current_queue().iter().enumerate().skip(1) {
            if len_queue > 10 && i > 8 {
                if i == len_queue {
                    ()
                } else if i == len_queue - 1 {
                    content = format!("{}\n\n     ...\n", content);
                    continue;
                } else {
                    continue;
                }
            }
            let metadata = handle.metadata();
            let title = match (&metadata.title, &metadata.source_url) {
                (Some(title), Some(url)) => format!("[{}]({})", title, url),
                (Some(title), None) => title.to_owned(),
                _ => "<UNKNOWN>".to_owned(),
            };
            content = format!("{}\n{:>3}: {}", content, i, title)
        }
        // content = format!("{}\n```", content);

        let builder = EmbedBuilder::new()
            .description(content)
            .title("CURRENT QUEUE:");
        let _ = command.create_message(&ctx, builder).await;
    }

    Ok(())
}
