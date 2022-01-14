use std::sync::Arc;

use twilight_model::application::{
    command::{
        CommandOption, CommandOptionValue as CommandOptionValueLiteral, NumberCommandOptionData,
    },
    interaction::{
        application_command::{CommandData, CommandOptionValue},
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
    async fn parse_options(_: Arc<Context>, data: &mut CommandData) -> BotResult<Self> {
        for option in data.options.iter() {
            if let CommandOptionValue::Integer(amount) = option.value {
                if option.name == "amount" {
                    return Ok(Self {
                        amount: amount as usize,
                    });
                }
            }
        }

        Ok(Self { amount: 1 })
    }
}

fn skip_options() -> Vec<CommandOption> {
    let option_data = NumberCommandOptionData {
        autocomplete: false,
        choices: vec![],
        description: "Specify a number of songs to skip, skips one by default".to_string(),
        max_value: None,
        min_value: Some(CommandOptionValueLiteral::Integer(1)),
        name: "amount".to_string(),
        required: false,
    };

    vec![CommandOption::Integer(option_data)]
}

pub async fn skip(ctx: Arc<Context>, command: ApplicationCommand, args: SkipArgs) -> BotResult<()> {
    let SkipArgs { amount } = args;
    info!("Skipping {} song(s) in song queue...", amount);
    if amount == 0 {
        let builder = MessageBuilder::new().error("Stop trying to break the bot.");
        return command.create_message(&ctx, builder).await;
    }

    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap().get()) {
        let call = call.lock().await;

        if call.queue().is_empty() {
            let builder = MessageBuilder::new().error("No song is currently playing!");
            return command.create_message(&ctx, builder).await;
        }

        for _ in 0..amount.min(call.queue().len()) {
            let success = call.queue().skip();

            if let Err(e) = success {
                let builder =
                    MessageBuilder::new().error("Failed to skip all of the songs! Blame Joshi :c");
                let _ = command.create_message(&ctx, builder).await;
                return Err(e.into());
            }
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
