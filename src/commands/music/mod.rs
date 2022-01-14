mod clear;
mod pause;
mod play;
mod queue;
mod skip;
mod stop;
mod volume;

pub use clear::Clear;
pub use pause::Pause;
pub use play::Play;
pub use queue::Queue;
pub use skip::Skip;
pub use stop::Stop;
pub use volume::Volume;

// use std::sync::Arc;

// use twilight_model::application::{
//     command::{ChoiceCommandOptionData, CommandOption, OptionsCommandOptionData},
//     interaction::{
//         application_command::{CommandData, CommandDataOption},
//         ApplicationCommand,
//     },
// };

// use crate::{context::Context, error::BotResult};

// #[command]
// #[args = "MusicArgs"]
// #[description = "Manipulate music"]
// #[options = "music_options"]
// pub struct Music;

// pub enum MusicArgs {
//     Clear,
//     Pause,
//     Play(String),
//     Queue,
//     Skip(usize),
//     Stop,
//     Volume(f32),
// }

// impl MusicArgs {
//     async fn parse_options(_: Arc<Context>, data: CommandData) -> BotResult<Self> {
//         for option in data.options {
//             if let CommandDataOption::SubCommand { name, options } = option {
//                 match name.as_str() {
//                     "clear" => return Ok(Self::Clear),
//                     "pause" => return Ok(Self::Pause),
//                     "play" => return Self::parse_play_options(options),
//                     "queue" => return Ok(Self::Queue),
//                     "skip" => return Self::parse_skip_options(options),
//                     "stop" => return Ok(Self::Stop),
//                     "volume" => return Self::parse_volume_options(options),
//                     _ => (),
//                 }
//             }
//         }

//         unreachable!();
//     }

//     fn parse_play_options(options: Vec<CommandDataOption>) -> BotResult<Self> {
//         for option in options {
//             if let CommandDataOption::String { name, value } = option {
//                 if name == "song" {
//                     return Ok(Self::Play(value));
//                 }
//             }
//         }

//         unreachable!()
//     }

//     fn parse_skip_options(options: Vec<CommandDataOption>) -> BotResult<Self> {
//         for option in options {
//             if let CommandDataOption::Integer { name, value } = option {
//                 if name == "amount" {
//                     return Ok(Self::Skip(value.max(0) as usize));
//                 }
//             }
//         }

//         Ok(Self::Skip(1))
//     }

//     fn parse_volume_options(options: Vec<CommandDataOption>) -> BotResult<Self> {
//         for option in options {
//             if let CommandDataOption::String { name, value } = option {
//                 if name == "volume" {
//                     let value = value.parse::<f32>()?;
//                     return Ok(Self::Volume(value.max(0.0)));
//                 }
//             }
//         }

//         unreachable!()
//     }
// }

// fn music_options() -> Vec<CommandOption> {
//     let clear = OptionsCommandOptionData {
//         description: "Clear the queue".to_string(),
//         name: "clear".to_string(),
//         options: vec![],
//         required: false,
//     };

//     let pause = OptionsCommandOptionData {
//         description: "Pause or unpause the song that's currently playing".to_string(),
//         name: "pause".to_string(),
//         options: vec![],
//         required: false,
//     };

//     let play_option = ChoiceCommandOptionData {
//         choices: vec![],
//         description: "Specify a song name or youtube url".to_string(),
//         name: "song".to_string(),
//         required: true,
//     };

//     let play = OptionsCommandOptionData {
//         description: "Play a given song".to_string(),
//         name: "play".to_string(),
//         options: vec![CommandOption::String(play_option)],
//         required: false,
//     };

//     let queue = OptionsCommandOptionData {
//         description: "Display the current song queue".to_string(),
//         name: "queue".to_string(),
//         options: vec![],
//         required: false,
//     };

//     let skip_option = ChoiceCommandOptionData {
//         choices: vec![],
//         description: "Specify a number of songs to skip, skips one by default".to_string(),
//         name: "amount".to_string(),
//         required: false,
//     };

//     let skip = OptionsCommandOptionData {
//         description: "Skip a number of songs in the queue".to_string(),
//         name: "skip".to_string(),
//         options: vec![CommandOption::Integer(skip_option)],
//         required: false,
//     };

//     let stop = OptionsCommandOptionData {
//         description: "Stop the currently playing song and clear the queue".to_string(),
//         name: "stop".to_string(),
//         options: vec![],
//         required: false,
//     };

//     let volume_option = ChoiceCommandOptionData {
//         choices: vec![],
//         description: "Specify the level to set the volume to (1 by default, can be decimal)"
//             .to_string(),
//         name: "volume".to_string(),
//         required: true,
//     };

//     let volume = OptionsCommandOptionData {
//         description: "Change the volume of the current song".to_string(),
//         name: "volume".to_string(),
//         options: vec![CommandOption::String(volume_option)],
//         required: false,
//     };

//     vec![
//         CommandOption::SubCommand(clear),
//         CommandOption::SubCommand(pause),
//         CommandOption::SubCommand(play),
//         CommandOption::SubCommand(queue),
//         CommandOption::SubCommand(skip),
//         CommandOption::SubCommand(stop),
//         CommandOption::SubCommand(volume),
//     ]
// }

// async fn music(ctx: Arc<Context>, command: ApplicationCommand, args: MusicArgs) -> BotResult<()> {
//     match args {
//         MusicArgs::Clear => clear::clear(ctx, command).await,
//         MusicArgs::Pause => pause::pause(ctx, command).await,
//         MusicArgs::Play(song) => play::play(ctx, command, song).await,
//         MusicArgs::Queue => queue::queue(ctx, command).await,
//         MusicArgs::Skip(amount) => skip::skip(ctx, command, amount).await,
//         MusicArgs::Stop => stop::stop(ctx, command).await,
//         MusicArgs::Volume(volume) => volume::volume(ctx, command, volume).await,
//     }
// }
