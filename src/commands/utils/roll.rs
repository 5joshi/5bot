use std::sync::Arc;

use rand::Rng;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{
        CommandOption, CommandOptionValue as CommandOptionValueLiteral, NumberCommandOptionData,
    },
    interaction::{
        application_command::{CommandData, CommandOptionValue},
        ApplicationCommand,
    },
};

use crate::{context::Context, error::BotResult};

#[command]
#[args = "RollArgs"]
#[description = "Roll a random number"]
#[options = "roll_options"]
pub struct Roll;

pub struct RollArgs {
    limit: u64,
}

impl RollArgs {
    async fn parse_options(_: Arc<Context>, data: &mut CommandData) -> BotResult<Self> {
        let mut limit: u64 = 100;
        for option in data.options.iter() {
            if let CommandOptionValue::Integer(value) = option.value {
                if option.name == "limit" {
                    limit = value.max(0) as u64;
                }
            }
        }
        Ok(Self { limit })
    }
}

fn roll_options() -> Vec<CommandOption> {
    let option_data = NumberCommandOptionData {
        autocomplete: false,
        choices: vec![],
        description: "Specify an upper limit, defaults to 100".to_string(),
        min_value: Some(CommandOptionValueLiteral::Integer(0)),
        max_value: None,
        name: "limit".to_string(),
        required: false,
    };

    vec![CommandOption::Integer(option_data)]
}

async fn roll(ctx: Arc<Context>, command: ApplicationCommand, args: RollArgs) -> BotResult<()> {
    let y = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..args.limit)
    };

    let response = InteractionResponse::ChannelMessageWithSource(CallbackData {
        allowed_mentions: None,
        components: None,
        content: Some(format!("You rolled {}! :game_die:", y)),
        embeds: vec![],
        flags: None,
        tts: None,
    });

    ctx.http
        .interaction_callback(command.id, &command.token, &response)
        .exec()
        .await?;

    Ok(())
}
