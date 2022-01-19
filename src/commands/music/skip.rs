use std::sync::Arc;

use songbird::tracks::TrackError;
use twilight_model::application::{
    command::{ChoiceCommandOptionData, CommandOption},
    interaction::{
        application_command::{CommandData, CommandDataOption},
        ApplicationCommand,
    },
};

use crate::{
    context::Context,
    error::BotResult,
    utils::{ApplicationCommandExt, MessageBuilder},
};

#[command]
#[args = "SkipArgs"]
#[description = "Skip a number of songs in the queue"]
#[options = "skip_options"]
pub struct Skip;

pub struct SkipArgs {
    amount: usize,
}

impl SkipArgs {
    async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
        for option in data.options {
            if let CommandDataOption::Integer { name, value } = option {
                if name == "amount" {
                    return Ok(Self {
                        amount: value.max(0) as usize,
                    });
                }
            }
        }

        Ok(Self { amount: 1 })
    }
}

fn skip_options() -> Vec<CommandOption> {
    let option_data = ChoiceCommandOptionData {
        choices: vec![],
        description: "Specify a number of songs to skip, skips one by default".to_string(),
        name: "amount".to_string(),
        required: false,
    };

    vec![CommandOption::Integer(option_data)]
}

pub async fn skip(ctx: Arc<Context>, command: ApplicationCommand, args: SkipArgs) -> BotResult<()> {
    let SkipArgs { amount } = args;
    if amount == 0 {
        let builder = MessageBuilder::new().error("Stop trying to break the bot.");
        return command.create_message(&ctx, builder).await;
    }

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

        info!("Skipping {} song(s) in song queue...", amount);
        let result = call.queue().modify_queue(|q| {
            for item in q.into_iter().take(amount) {
                item.stop()?;
            }
            q.rotate_left(amount);
            q.truncate(q.len().saturating_sub(amount));
            if let Some(item) = q.front() {
                item.play()?;
            }
            Ok::<_, TrackError>(())
        });

        if let Err(e) = result {
            let builder =
                MessageBuilder::new().error("Failed to skip all of the songs! Blame Joshi :c");
            let _ = command.create_message(&ctx, builder).await;
            return Err(e.into());
        }

        let content = format!(
            "Skipped {} song{}!",
            amount,
            if amount != 1 { "s" } else { "" }
        );
        let builder = MessageBuilder::new().embed(content);
        let _ = command.create_message(&ctx, builder).await;
    }

    Ok(())
}
