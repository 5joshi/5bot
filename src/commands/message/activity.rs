use std::{borrow::Cow, sync::Arc};

use twilight_model::{
    application::{
        command::{BaseCommandOptionData, ChannelCommandOptionData, CommandOption},
        interaction::{
            application_command::{CommandData, InteractionChannel},
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
#[args = "ActivityArgs"]
#[description = "Get the server activity for the last month, week, day and hour"]
#[options = "activity_options"]
pub struct Activity;

pub struct ActivityArgs {
    channel: Option<InteractionChannel>,
}

impl ActivityArgs {
    async fn parse_options(_: Arc<Context>, data: &mut CommandData) -> BotResult<Self> {
        let channel = data.resolved.and_then(|mut data| data.channels.pop());
        Ok(Self { channel })
    }
}

fn activity_options() -> Vec<CommandOption> {
    let option_data = ChannelCommandOptionData {
        description: "Specify an optional channel to check the activity for".to_string(),
        name: "channel".to_string(),
        required: false,
    };

    vec![CommandOption::Channel(option_data)]
}

async fn activity(
    ctx: Arc<Context>,
    command: ApplicationCommand,
    args: ActivityArgs,
) -> BotResult<()> {
    let channel_opt = args
        .channel
        .as_ref()
        .filter(|channel| channel.kind != ChannelType::GuildText);

    let guild_id = if let Some(id) = command.guild_id {
        id
    } else {
        let builder = MessageBuilder::new().error("This command can only be used in a server!");
        command.create_message(&ctx, builder).await?;
        return Ok(());
    };

    let is_channel = args.channel.is_some();

    if channel_opt.is_some() {
        let builder = MessageBuilder::new().error("Please specify a regular text channel!");
        command.create_message(&ctx, builder).await?;
    } else {
        let activity = ctx
            .database
            .get_activity(args.channel.as_ref().map(|c| c.id))
            .await?;

        let name = args
            .channel
            .map(|c| c.name)
            .or_else(|| ctx.cache.guild(guild_id).map(|g| g.name))
            .map(Cow::from)
            .unwrap_or_else(|| "<NAME NOT FOUND>".into());

        let builder = create_activity_embed(activity, name, is_channel);
        command.create_message(&ctx, builder).await?;
    }

    Ok(())
}

fn create_activity_embed(activity: MessageActivity, name: Cow<str>, channel: bool) -> EmbedBuilder {
    let title = format!(
        "Message activity in {}{}:",
        if channel { "channel #" } else { "server " },
        name,
    );

    // Get the length required for # messages
    let amount_len = 9.max(with_comma_uint(activity.month).to_string().len());

    // Get the activity strings along with their max length
    let activity_hour = format!(
        "{}%",
        round((100 * 24 * 30 * activity.hour) as f32 / activity.month as f32)
    );
    let activity_day = format!(
        "{}%",
        round((100 * 30 * activity.day) as f32 / activity.month as f32)
    );
    let activity_week = format!(
        "{}%",
        round(100.0 * 4.286 * activity.week as f32 / activity.month as f32)
    );

    let activity_len = 8
        .max(activity_hour.len())
        .max(activity_day.len())
        .max(activity_week.len());

    let description = format!(
        "```\n \
        Last | {:>len1$} |   Total | {:>len2$}\n\
        ------+-{dash:->len1$}-+---------+-{dash:->len2$}\n \
        Hour | {:>len1$} | {:>6}% | {:>len2$}\n  \
        Day | {:>len1$} | {:>6}% | {:>len2$}\n \
        Week | {:>len1$} | {:>6}% | {:>len2$}\n\
        Month | {:>len1$} | {:>6}% | {:>len2$}\n\
        ```",
        "#Messages",
        "Activity",
        with_comma_uint(activity.hour).to_string(),
        round(100.0 * activity.hour as f32 / activity.month as f32),
        activity_hour,
        with_comma_uint(activity.day).to_string(),
        round(100.0 * activity.day as f32 / activity.month as f32),
        activity_day,
        with_comma_uint(activity.week).to_string(),
        round(100.0 * activity.week as f32 / activity.month as f32),
        activity_week,
        with_comma_uint(activity.month).to_string(),
        100,
        "100%",
        dash = "-",
        len1 = amount_len,
        len2 = activity_len,
    );

    EmbedBuilder::new().title(title).description(description)
}

#[derive(Default)]
pub struct MessageActivity {
    pub month: usize,
    pub week: usize,
    pub day: usize,
    pub hour: usize,
}
