use std::fmt::Write;
use std::{borrow::Cow, sync::Arc};

use cow_utils::CowUtils;
use markov::Chain;
use twilight_model::id::{ChannelId, UserId};
use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, CommandOption,
        },
        interaction::{
            application_command::{CommandData, CommandDataOption, InteractionChannel},
            ApplicationCommand,
        },
    },
    channel::ChannelType,
};

use crate::{
    context::Context,
    error::BotResult,
    utils::{
        numbers::{round, with_comma_uint},
        ApplicationCommandExt, EmbedBuilder, MessageBuilder,
    },
};

#[command]
#[args = "ImpersonateArgs"]
#[description = "Impersonate a user or channel based on previous message data"]
#[options = "impersonate_options"]
pub struct Impersonate;

pub struct ImpersonateArgs {
    author: Option<UserId>,
    channel: Option<ChannelId>,
}

impl ImpersonateArgs {
    async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
        let author = data
            .resolved
            .as_ref()
            .and_then(|data| data.members.last().map(|c| c.id));
        let channel = data
            .resolved
            .as_ref()
            .and_then(|data| data.channels.last().map(|c| c.id));

        return Ok(Self { author, channel });
    }
}

fn impersonate_options() -> Vec<CommandOption> {
    let author = BaseCommandOptionData {
        description: "Specify an optional user to take message data from".to_string(),
        name: "author".to_string(),
        required: false,
    };

    let channel = ChannelCommandOptionData {
        channel_types: vec![ChannelType::GuildText],
        description: "Specify an optional channel to take message data from".to_string(),
        name: "channel".to_string(),
        required: false,
    };

    vec![CommandOption::User(author), CommandOption::Channel(channel)]
}

async fn impersonate(
    ctx: Arc<Context>,
    command: ApplicationCommand,
    args: ImpersonateArgs,
) -> BotResult<()> {
    command.start_thinking(&ctx).await?;

    let strings = ctx
        .database
        .get_messages(args.author, args.channel, command.guild_id.unwrap())
        .await?;

    if strings.is_empty() {
        let builder = MessageBuilder::new().error(format!(
            "I haven't seen any messages{}{}!",
            args.author.map_or_else(|| "", |_| " from this user"),
            args.channel.map_or_else(|| "", |_| " in this channel")
        ));
        return command.update_message(&ctx, builder).await;
    }

    let mut chain: Chain<String> = Chain::new();
    for s in strings.iter() {
        chain.feed_str(&s.cow_to_lowercase());
    }
    let mut content = String::new();
    for line in chain.str_iter_for(strings.len().min(15)) {
        let _ = writeln!(content, "{line}");
    }
    let builder = MessageBuilder::new().embed(content);
    return command.update_message(&ctx, builder).await;
}
