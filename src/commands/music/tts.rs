use std::{env, sync::Arc};

use cow_utils::CowUtils;
use reqwest::header::AUTHORIZATION;
use songbird::{
    input::{Codec, Container, Input, Reader, Restartable},
    Event, EventContext, EventHandler, TrackEvent,
};
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    time::{self, Duration},
};
use twilight_model::{
    application::{
        command::{ChoiceCommandOptionData, CommandOption, CommandOptionChoice},
        interaction::{
            application_command::{CommandData, CommandDataOption},
            ApplicationCommand,
        },
    },
    gateway::presence::{ActivityType, Status},
};

use crate::{
    context::Context,
    error::BotResult,
    utils::{
        matcher, ApplicationCommandExt, BatchGetResponse, EmbedBuilder, MessageBuilder,
        SpeakResponse, SpeakStatusResponse, UBERDUCK_BASE,
    },
};

#[command]
#[args = "TtsArgs"]
#[description = "Say something in a chosen voice"]
#[options = "tts_options"]
pub struct Tts;

pub struct TtsArgs {
    text: String,
    voice: String,
}

impl TtsArgs {
    async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
        let mut text = "".to_string();
        let mut voice = "ye".to_string();
        for option in data.options {
            if let CommandDataOption::String { name, value } = option {
                if name == "text" {
                    text = value;
                } else if name == "voice" {
                    voice = value;
                }
            }
        }
        Ok(Self {
            text: text.to_string(),
            voice: voice.to_string(),
        })
    }
}

fn tts_options() -> Vec<CommandOption> {
    let text_data = ChoiceCommandOptionData {
        choices: vec![],
        description: "Specify what you want the bot to say".to_string(),
        name: "text".to_string(),
        required: true,
    };

    let voice_data = ChoiceCommandOptionData {
        choices: vec![
            CommandOptionChoice::String {
                name: "Dwayne \"The Rock\" Johnson".to_string(),
                value: "the-rock".to_string(),
            },
            CommandOptionChoice::String {
                name: "Moistcr1tikal".to_string(),
                value: "cr1tikal".to_string(),
            },
            CommandOptionChoice::String {
                name: "BTMC".to_string(),
                value: "btmc".to_string(),
            },
            CommandOptionChoice::String {
                name: "Spongebob Squarepants".to_string(),
                value: "spongebob".to_string(),
            },
            CommandOptionChoice::String {
                name: "Matpat".to_string(),
                value: "matpat".to_string(),
            },
            CommandOptionChoice::String {
                name: "Kurzgesagt".to_string(),
                value: "kurzgesagt".to_string(),
            },
            CommandOptionChoice::String {
                name: "Kanye West".to_string(),
                value: "ye".to_string(),
            },
            CommandOptionChoice::String {
                name: "Goku".to_string(),
                value: "goku".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Arnold Schwarzenegger".to_string(),
            //     value: "arnold-schwarzenegger".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Peter Griffin".to_string(),
                value: "peter-griffin".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Patrick".to_string(),
            //     value: "patrick".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Eminem".to_string(),
                value: "eminem".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Gordon Ramsay".to_string(),
            //     value: "gordon-ramsay".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Kermit The Frog".to_string(),
            //     value: "kermit-the-frog".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Leafyishere".to_string(),
            //     value: "leafyishere".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Mark Zuckerberg".to_string(),
            //     value: "mark-zuckerberg".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Vsauce".to_string(),
            //     value: "michaelstevens".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Mickey Mouse".to_string(),
                value: "mickey-mouse".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "MrBeast".to_string(),
            //     value: "mrbeast".to_string(),
            // },
            CommandOptionChoice::String {
                name: "The Weeknd".to_string(),
                value: "the-weeknd".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Walter White".to_string(),
            //     value: "walter-white".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Ben Shapiro".to_string(),
                value: "benshapiro".to_string(),
            },
            CommandOptionChoice::String {
                name: "Cookie Masterson".to_string(),
                value: "cookie-masterson".to_string(),
            },
            CommandOptionChoice::String {
                name: "Morty".to_string(),
                value: "morty".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Morgan Freeman".to_string(),
            //     value: "morgan-freeman".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Michael Caine".to_string(),
            //     value: "michael-caine".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Siri".to_string(),
                value: "siri-female-british".to_string(),
            },
            CommandOptionChoice::String {
                name: "Benedict Cumberbatch".to_string(),
                value: "benedict-cumberbatch".to_string(),
            },
            CommandOptionChoice::String {
                name: "Alex Jones".to_string(),
                value: "alex-jones".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Kratos".to_string(),
            //     value: "kratos".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Google Assistant".to_string(),
            //     value: "google-assistant".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Agent 47".to_string(),
            //     value: "hitman-agent-47".to_string(),
            // },
            CommandOptionChoice::String {
                name: "Stan Lee".to_string(),
                value: "stan-lee".to_string(),
            },
            CommandOptionChoice::String {
                name: "Naruto".to_string(),
                value: "naruto-uzumaki".to_string(),
            },
            // CommandOptionChoice::String {
            //     name: "Professor Layton".to_string(),
            //     value: "layton".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Al Michaels".to_string(),
            //     value: "al-michaels".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Kevin Harlan".to_string(),
            //     value: "kevin-harlan".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Marge Simpson".to_string(),
            //     value: "marge-simpson".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Homer Simpson".to_string(),
            //     value: "homer-simpson".to_string(),
            // },
            // CommandOptionChoice::String {
            //     name: "Cypher".to_string(),
            //     value: "cypher-valorant".to_string(),
            // },
            CommandOptionChoice::String {
                name: "3kliksphilip".to_string(),
                value: "3kliksphilip".to_string(),
            },
            CommandOptionChoice::String {
                name: "Linus Tech Tips".to_string(),
                value: "linustt".to_string(),
            },
            CommandOptionChoice::String {
                name: "Dhar Mann".to_string(),
                value: "dharr-mann".to_string(),
            },
            CommandOptionChoice::String {
                name: "GradeAUnderA".to_string(),
                value: "gradeaundera".to_string(),
            },
            CommandOptionChoice::String {
                name: "Pishifat".to_string(),
                value: "pishifat".to_string(),
            },
        ],
        description: "Specify the voice the bot should use".to_string(),
        name: "voice".to_string(),
        required: true,
    };

    vec![
        CommandOption::String(text_data),
        CommandOption::String(voice_data),
    ]
}

pub async fn tts(ctx: Arc<Context>, command: ApplicationCommand, args: TtsArgs) -> BotResult<()> {
    command.start_thinking(&ctx).await?;

    let author_id = command.user_id()?;
    let guild_id = command.guild_id.expect("Missing Guild ID for play command");

    let channel_id = match ctx
        .cache
        .voice_state(author_id, guild_id)
        .and_then(|state| state.channel_id)
    {
        Some(id) => id,
        None => {
            let builder = MessageBuilder::new().error("You aren't in a voice channel!");
            return command.update_message(&ctx, builder).await;
        }
    };

    let (_handle, success) = ctx.songbird.join(guild_id, channel_id).await;

    if let Err(success) = success {
        let builder = MessageBuilder::new().error("Failed to join voice channel! Blame Joshi :c");
        let _ = command.update_message(&ctx, builder).await;
        return Err(success.into());
    }

    info!(
        "Joined channel {} after tts command by {}",
        if let Some(channel) = ctx.cache.guild_channel(channel_id) {
            channel.name().to_owned()
        } else {
            channel_id.to_string()
        },
        command.username()?
    );

    let TtsArgs { text, voice } = args;
    let req = format!("{}speak", UBERDUCK_BASE);
    let body = format!(
        "{{\"speech\": \"{}\", \"voice\": \"{}\"}}",
        text.cow_replace("\"", "\\\""),
        voice
    );
    info!("{}", body);
    let bytes = ctx
        .client
        .post(req)
        .basic_auth(
            env::var("UBERDUCK_API_KEY").expect("Missing environment variable (UBERDUCK_API_KEY)."),
            Some(
                env::var("UBERDUCK_SECRET")
                    .expect("Missing environment variable (UBERDUCK_SECRET)."),
            ),
        )
        .body(body)
        .send()
        .await?
        .bytes()
        .await?;
    // info!("{}", String::from_utf8_lossy(&bytes));
    let uuid = serde_json::from_slice::<SpeakResponse>(&bytes)?.uuid;
    let req = format!("{}speak-status?uuid={}", UBERDUCK_BASE, uuid);
    let mut audio_path = None;

    let mut interval = time::interval(Duration::from_secs(1));
    let mut tries = 0;
    while audio_path.is_none() && tries < 30 {
        interval.tick().await;
        let bytes = ctx.client.get(&req).send().await?.bytes().await?;
        audio_path = serde_json::from_slice::<SpeakStatusResponse>(&bytes)?.path;
        tries += 1;
    }

    let path = format!("./tts/{}.wav", uuid);
    let url = match audio_path {
        Some(url) => url,
        None => {
            let builder = MessageBuilder::new().error("Failed to generate audio! Blame Joshi :c");
            return command.update_message(&ctx, builder).await;
        }
    };

    let bytes = ctx.client.get(&url).send().await?.bytes().await?;
    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    match Restartable::ffmpeg(path.clone(), false).await {
        Ok(audio) => {
            let input = Input::from(audio);

            if let Some(call_lock) = ctx.songbird.get(guild_id) {
                let mut call = call_lock.lock().await;
                let empty = call.queue().is_empty();

                let content = format!(
                    "{}\"{}\" in the voice of {}{}",
                    if empty {
                        "Started saying "
                    } else {
                        "Added TTS "
                    },
                    text,
                    voice,
                    if empty { "" } else { " to the queue" },
                );

                info!("{}", content);

                let mut builder = EmbedBuilder::new().description(content);

                call.enqueue_source(input);
                call.queue().modify_queue(|q| {
                    q.back()
                        .map(|q| q.add_event(Event::Track(TrackEvent::End), TrackEnd(path)))
                });
                command.update_message(&ctx, builder).await?;

                // ctx.trackdata.write().replace(handle);
            }
        }
        Err(e) => {
            tokio::fs::remove_file(path).await?;
            unwind_error!(
                error,
                e,
                "Couldn't generate ffmpeg audio input from {}: {}",
                uuid
            );
            let builder = MessageBuilder::new().embed("Failed to generate audio! Blame Joshi :c");
            command.update_message(&ctx, builder).await?;
        }
    }

    Ok(())
}

struct TrackEnd(String);

#[async_trait]
impl EventHandler for TrackEnd {
    async fn act(&self, ctx: &songbird::EventContext<'_>) -> Option<songbird::Event> {
        tokio::fs::remove_file(&self.0).await;

        None
    }
}
