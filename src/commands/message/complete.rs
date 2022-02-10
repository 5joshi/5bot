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
#[args = "CompleteArgs"]
#[description = "Finish the given sentence based on previous message data"]
#[options = "complete_options"]
pub struct Complete;

pub struct CompleteArgs {
    author: Option<UserId>,
    channel: Option<ChannelId>,
    contains: String,
}

impl CompleteArgs {
    async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
        let author = data
            .resolved
            .as_ref()
            .and_then(|data| data.members.last().map(|c| c.id));
        let channel = data
            .resolved
            .as_ref()
            .and_then(|data| data.channels.last().map(|c| c.id));

        for option in data.options {
            if let CommandDataOption::String { name, value } = option {
                if name == "contains" {
                    return Ok(Self {
                        author,
                        channel,
                        contains: value,
                    });
                }
            }
        }

        unreachable!()
    }
}

fn complete_options() -> Vec<CommandOption> {
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

    let contains = ChoiceCommandOptionData {
        choices: Vec::new(),
        description: "Specify the sentence to complete".to_string(),
        name: "contains".to_string(),
        required: true,
    };

    vec![
        CommandOption::String(contains),
        CommandOption::User(author),
        CommandOption::Channel(channel),
    ]
}

async fn complete(
    ctx: Arc<Context>,
    command: ApplicationCommand,
    args: CompleteArgs,
) -> BotResult<()> {
    command.start_thinking(&ctx).await?;

    let strings = ctx
        .database
        .get_complete_messages(
            args.author,
            args.channel,
            &args.contains,
            command.guild_id.unwrap(),
        )
        .await?;

    if strings.is_empty() {
        let builder =
            MessageBuilder::new().error("I haven't seen any messages containing this string yet!");
        return command.update_message(&ctx, builder).await;
    }

    let mut chain: Chain<String> = Chain::new();
    let len = args.contains.len();
    for s in strings.iter() {
        let s = s.cow_to_lowercase();
        if let Some(idx) = s.find(&args.contains) {
            let suffix = &s[idx + len..];
            chain.feed_str(suffix);
        }
    }
    let mut content = String::new();
    for line in chain.str_iter_for(strings.len().min(15)) {
        let _ = writeln!(content, "{}{line}", args.contains);
    }
    let builder = MessageBuilder::new().embed(content);
    return command.update_message(&ctx, builder).await;
}
