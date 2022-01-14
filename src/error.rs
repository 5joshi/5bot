use chrono::ParseError;
use irc::error::Error as IrcError;
use rosu_v2::prelude::OsuError;
use songbird::error::JoinError;
use songbird::tracks::TrackError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlError;
use std::error::Error as StdError;
use std::fmt;
use std::num::ParseFloatError;
use twilight_gateway::cluster::{ClusterCommandError, ClusterStartError};
use twilight_http::request::application::interaction::update_original_response::UpdateOriginalResponseError;
use twilight_http::request::application::InteractionError;
use twilight_http::request::prelude::create_message::CreateMessageError;
use twilight_http::response::DeserializeBodyError;
use twilight_http::Error as TwilightHttpError;
use twilight_model::application::interaction::ApplicationCommand;

pub type BotResult<T> = Result<T, Error>;

#[rustfmt::skip]
#[derive(Debug)]
pub enum Error {
    ClusterCommand { src: ClusterCommandError },
    ClusterStart { src: ClusterStartError },
    Command { name: &'static str, src: Box<Error> },
    CreateMessage { src: CreateMessageError },
    DeserializeBody { src: DeserializeBodyError },
    Interaction { src: InteractionError },
    Irc { src: IrcError },
    JoinVoicechat { src: JoinError },
    Migration { src: MigrateError },
    MissingSlashAuthor,
    Osu { src: OsuError },
    ParseFloat { src: ParseFloatError },
    ParseTime { src: ParseError },
    SongbirdTrack { src: TrackError },
    Sql { src: SqlError },
    TwilightHttp { src: TwilightHttpError },
    UnknownInteraction { command: Box<ApplicationCommand> },
    UpdateOriginalResponse { src: UpdateOriginalResponseError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ClusterCommand { .. } => f.write_str("Error occurred on cluster request."),
            Error::ClusterStart { .. } => f.write_str("Failed to start cluster."),
            Error::Command { name, .. } => write!(f, "Failed to execute command ({})", name),
            Error::CreateMessage { .. } => f.write_str("Failed to create message."),
            Error::DeserializeBody { .. } => f.write_str("Failed to deserialize Discord object."),
            Error::Interaction { .. } => f.write_str("Failed to interact with Discord."),
            Error::Irc { .. } => f.write_str("Failed to communicate with osu! IRC."),
            Error::JoinVoicechat { .. } => f.write_str("Failed to join discord voicechat."),
            Error::Migration { .. } => f.write_str("Failed to migrate database."),
            Error::MissingSlashAuthor => f.write_str("Slash author was not found."),
            Error::Osu { .. } => f.write_str("Failed to communicate with osu! API."),
            Error::ParseFloat { .. } => f.write_str("Failed to parse float."),
            Error::ParseTime { .. } => f.write_str("Failed to parse timestamp with chrono."),
            Error::SongbirdTrack { .. } => {
                f.write_str("Error when using method on songbird track.")
            }
            Error::Sql { .. } => f.write_str("Error caused by database."),
            Error::TwilightHttp { .. } => f.write_str("Error while using Twilight HTTP."),
            Error::UnknownInteraction { command } => {
                write!(
                    f,
                    "Received unknown interaction ({}): {:#?}",
                    command.data.name, command
                )
            }
            Error::UpdateOriginalResponse { .. } => {
                f.write_str("Error while updating original response.")
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ClusterCommand { src } => Some(src),
            Error::ClusterStart { src } => Some(src),
            Error::Command { src, .. } => Some(src),
            Error::CreateMessage { src } => Some(src),
            Error::DeserializeBody { src } => Some(src),
            Error::Interaction { src } => Some(src),
            Error::Irc { src } => Some(src),
            Error::JoinVoicechat { src } => Some(src),
            Error::Osu { src } => Some(src),
            Error::ParseFloat { src } => Some(src),
            Error::ParseTime { src } => Some(src),
            Error::Migration { src } => Some(src),
            Error::MissingSlashAuthor => None,
            Error::SongbirdTrack { src } => Some(src),
            Error::Sql { src } => Some(src),
            Error::TwilightHttp { src } => Some(src),
            Error::UnknownInteraction { .. } => None,
            Error::UpdateOriginalResponse { src } => Some(src),
        }
    }
}

impl From<ClusterCommandError> for Error {
    fn from(src: ClusterCommandError) -> Self {
        Self::ClusterCommand { src }
    }
}

impl From<ClusterStartError> for Error {
    fn from(src: ClusterStartError) -> Self {
        Self::ClusterStart { src }
    }
}

impl From<CreateMessageError> for Error {
    fn from(src: CreateMessageError) -> Self {
        Self::CreateMessage { src }
    }
}

impl From<DeserializeBodyError> for Error {
    fn from(src: DeserializeBodyError) -> Self {
        Self::DeserializeBody { src }
    }
}

impl From<InteractionError> for Error {
    fn from(src: InteractionError) -> Self {
        Self::Interaction { src }
    }
}

impl From<IrcError> for Error {
    fn from(src: IrcError) -> Self {
        Self::Irc { src }
    }
}

impl From<JoinError> for Error {
    fn from(src: JoinError) -> Self {
        Self::JoinVoicechat { src }
    }
}

impl From<MigrateError> for Error {
    fn from(src: MigrateError) -> Self {
        Self::Migration { src }
    }
}

impl From<OsuError> for Error {
    fn from(src: OsuError) -> Self {
        Self::Osu { src }
    }
}

impl From<ParseFloatError> for Error {
    fn from(src: ParseFloatError) -> Self {
        Self::ParseFloat { src }
    }
}

impl From<ParseError> for Error {
    fn from(src: ParseError) -> Self {
        Self::ParseTime { src }
    }
}

impl From<TrackError> for Error {
    fn from(src: TrackError) -> Self {
        Self::SongbirdTrack { src }
    }
}

impl From<SqlError> for Error {
    fn from(src: SqlError) -> Self {
        Self::Sql { src }
    }
}

impl From<TwilightHttpError> for Error {
    fn from(src: TwilightHttpError) -> Self {
        Self::TwilightHttp { src }
    }
}

impl From<UpdateOriginalResponseError> for Error {
    fn from(src: UpdateOriginalResponseError) -> Self {
        Self::UpdateOriginalResponse { src }
    }
}
