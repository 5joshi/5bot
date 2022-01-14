use std::sync::Arc;

use twilight_model::application::{
    command::{
        CommandOption, CommandOptionValue as CommandOptionValueLiteral, Number,
        NumberCommandOptionData,
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
#[args = "VolumeArgs"]
#[description = "Change the volume of the current song"]
#[options = "volume_options"]
pub struct Volume;

pub struct VolumeArgs {
    volume: f64,
}

impl VolumeArgs {
    async fn parse_options(_: Arc<Context>, data: &mut CommandData) -> BotResult<Self> {
        for option in data.options.iter() {
            if let CommandOptionValue::Number(Number(volume)) = option.value {
                if option.name == "volume" {
                    return Ok(Self { volume });
                }
            }
        }

        unreachable!()
    }
}

fn volume_options() -> Vec<CommandOption> {
    let option_data = NumberCommandOptionData {
        autocomplete: false,
        choices: vec![],
        description: "Specify the level to set the volume to (1 by default, can be decimal)"
            .to_string(),
        max_value: None,
        min_value: Some(CommandOptionValueLiteral::Number(Number(0.0))),
        name: "volume".to_string(),
        required: true,
    };

    vec![CommandOption::Number(option_data)]
}

pub async fn volume(
    ctx: Arc<Context>,
    command: ApplicationCommand,
    args: VolumeArgs,
) -> BotResult<()> {
    let VolumeArgs { volume } = args;
    info!("Setting song volume to {}...", volume);
    if let Some(call) = ctx.songbird.get(command.guild_id.unwrap().get()) {
        let call = call.lock().await;

        let handle = match call.queue().current() {
            Some(handle) => handle,
            None => {
                let builder = MessageBuilder::new().error("No song is currently playing!");
                return command.create_message(&ctx, builder).await;
            }
        };

        handle.set_volume(volume as f32);

        let content = format!("Changed volume to {}!", volume);
        let builder = MessageBuilder::new().embed(content);
        return command.create_message(&ctx, builder).await;
    }

    Ok(())
}
