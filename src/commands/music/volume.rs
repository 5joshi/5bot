use std::sync::Arc;

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
#[args = "VolumeArgs"]
#[description = "Change the volume of the current song"]
#[options = "volume_options"]
pub struct Volume;

pub struct VolumeArgs {
    volume: f32,
}

impl VolumeArgs {
    async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
        for option in data.options {
            if let CommandDataOption::String { name, value } = option {
                if name == "volume" {
                    let value = value.parse::<f32>()?;
                    return Ok(Self {
                        volume: value.max(0.0),
                    });
                }
            }
        }

        unreachable!()
    }
}

fn volume_options() -> Vec<CommandOption> {
    let option_data = ChoiceCommandOptionData {
        choices: vec![],
        description: "Specify the level to set the volume to (1 by default, can be decimal)"
            .to_string(),
        name: "volume".to_string(),
        required: true,
    };

    vec![CommandOption::String(option_data)]
}

pub async fn volume(
    ctx: Arc<Context>,
    command: ApplicationCommand,
    args: VolumeArgs,
) -> BotResult<()> {
    let VolumeArgs { volume } = args;
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

        let handle = match call.queue().current() {
            Some(handle) => handle,
            None => {
                let builder = MessageBuilder::new().error("No song is currently playing!");
                return command.create_message(&ctx, builder).await;
            }
        };

        info!("Setting song volume to {}...", volume);
        handle.set_volume(volume);

        let content = format!("Changed volume to {}!", volume);
        let builder = MessageBuilder::new().embed(content);
        return command.create_message(&ctx, builder).await;
    }

    Ok(())
}
