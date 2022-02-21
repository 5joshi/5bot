use chrono::ParseError;
use irc::error::Error as IrcError;
use regex::Error as RegexError;
use reqwest::Error as ReqwestError;
use rosu_pp::ParseError as RosuParseError;
use rosu_v2::prelude::OsuError;
use serde_json::error::Error as JsonError;
use songbird::error::JoinError;
use songbird::tracks::TrackError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use twilight_gateway::cluster::{ClusterCommandError, ClusterStartError};
use twilight_http::request::application::interaction::update_original_response::UpdateOriginalResponseError;
use twilight_http::request::application::InteractionError;
use twilight_http::request::prelude::create_message::CreateMessageError;
use twilight_http::response::DeserializeBodyError;
use twilight_http::Error as TwilightHttpError;
use twilight_model::application::interaction::ApplicationCommand;

pub type BotResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error occurred on cluster request.")]
    ClusterCommand(#[from] ClusterCommandError),
    #[error("Failed to start cluster.")]
    ClusterStart(#[from] ClusterStartError),
    #[error("Failed to execute command ({name})")]
    Command {
        name: &'static str,
        #[source]
        src: Box<Error>,
    },
    #[error("Failed to create message.")]
    CreateMessage(#[from] CreateMessageError),
    #[error("Failed to deserialize Discord object.")]
    DeserializeBody(#[from] DeserializeBodyError),
    #[error("Failed to interact with Discord.")]
    Interaction(#[from] InteractionError),
    #[error("I/O error.")]
    Io(#[from] IoError),
    #[error("Failed to communicate with osu! IRC.")]
    Irc(#[from] IrcError),
    #[error("Failed to join discord voicechat.")]
    JoinVoicechat(#[from] JoinError),
    #[error("Error while handling json.")]
    Json(#[from] JsonError),
    #[error("Error while downloading map.")]
    MapDownload(#[from] MapDownloadError),
    #[error("Failed to migrate database.")]
    Migration(#[from] MigrateError),
    #[error("Slash author was not found.")]
    MissingSlashAuthor,
    #[error("Failed to communicate with osu! API.")]
    Osu(#[from] OsuError),
    #[error("Failed to parse float.")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Failed to parse timestamp with chrono.")]
    ParseTime(#[from] ParseError),
    #[error("Regex error")]
    Regex(#[from] RegexError),
    #[error("Reqwest error.")]
    Reqwest(#[from] ReqwestError),
    #[error("Error when parsing with rosu.")]
    RosuParse(#[from] RosuParseError),
    #[error("Error when using method on songbird track.")]
    SongbirdTrack(#[from] TrackError),
    #[error("Error caused by database.")]
    Sql(#[from] SqlError),
    #[error("Error while using Twilight HTTP.")]
    TwilightHttp(#[from] TwilightHttpError),
    #[error("Received unknown interaction ({}): {command:#?}", .command.data.name)]
    UnknownInteraction { command: Box<ApplicationCommand> },
    #[error("Error while updating original response.")]
    UpdateOriginalResponse(#[from] UpdateOriginalResponseError),
}

#[derive(Debug, thiserror::Error)]
pub enum MapDownloadError {
    #[error("Reqwest error.")]
    Reqwest(#[from] ReqwestError),
    #[error("I/O error.")]
    Io(#[from] IoError),
    #[error("Failed to download map id {0}.")]
    RetryLimit(u32),
}
