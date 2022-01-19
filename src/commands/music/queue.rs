use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, EmbedBuilder, MessageBuilder, NUMBER_EMOTES},
};

#[command]
#[description = "Display the current song queue"]
pub struct Queue;

pub async fn queue(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let guild_id = command.guild_id.expect("Missing Guild ID for play command");

    if let Some(call) = ctx.songbird.get(guild_id) {
        let call = call.lock().await;

        info!("Displaying current song queue...");
        let len_queue = call.queue().len() as u32;

        if len_queue < 2 {
            let builder = MessageBuilder::new()
                .embed("The queue is currently empty!\nAdd more songs by using /play");
            return command.create_message(&ctx, builder).await;
        }

        let mut content = String::new();
        for (handle, i) in call.queue().current_queue().iter().zip(1..) {
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
            content = format!(
                "{}\n{:>3}: {}",
                content,
                DigitIter::new(i - 1).to_emotes(),
                title
            )
        }
        // content = format!("{}\n```", content);

        let builder = EmbedBuilder::new()
            .description(content)
            .title("CURRENT QUEUE:");
        let _ = command.create_message(&ctx, builder).await;
    }

    Ok(())
}

struct DigitIter {
    num: u32,
    trailing_zeros: u32,
}

impl DigitIter {
    fn new(mut n: u32) -> Self {
        let mut rev = 0;
        let mut trailing_zeros = 0;

        if n == 0 {
            return Self {
                num: 0,
                trailing_zeros: 1,
            };
        }

        while n % 10 == 0 {
            trailing_zeros += 1;
            n /= 10;
        }

        while n > 0 {
            rev *= 10;
            rev += n % 10;
            n /= 10;
        }

        Self {
            num: rev,
            trailing_zeros,
        }
    }

    fn to_emotes(self) -> String {
        if self.num == 0 {
            return ":musical_note:".to_owned();
        }
        let mut result = String::new();
        for digit in self {
            result += NUMBER_EMOTES[digit as usize];
        }

        result
    }
}

impl Iterator for DigitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num > 0 {
            let digit = self.num % 10;
            self.num /= 10;

            Some(digit)
        } else if self.trailing_zeros > 0 {
            self.trailing_zeros -= 1;

            Some(0)
        } else {
            None
        }
    }
}
