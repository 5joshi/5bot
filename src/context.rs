use std::sync::Arc;

use crate::{osu_irc::IrcClient, stats::BotStats};
use crate::{BotResult, Database};

use hashbrown::HashSet;
use parking_lot::RwLock;
use reqwest::Client;
use rosu_v2::Osu as OsuClient;
use songbird::Songbird;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Cluster;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::UpdatePresence;
use twilight_model::gateway::presence::{Activity, ActivityType, Status};
use twilight_model::id::GuildId;
use twilight_standby::Standby;

pub struct Context {
    pub cache: InMemoryCache,
    pub client: Client,
    pub database: Database,
    pub osu: OsuClient,
    pub irc: IrcClient,
    pub cluster: Cluster,
    pub http: HttpClient,
    // pub hub: Sheets,
    pub servers: RwLock<HashSet<GuildId>>,
    pub songbird: Songbird,
    pub standby: Standby,
    pub stats: BotStats,
}

impl Context {
    pub async fn set_shard_activity(
        &self,
        shard_id: u64,
        status: Status,
        activity_type: ActivityType,
        message: impl Into<String>,
    ) -> BotResult<()> {
        let activities = vec![generate_activity(activity_type, message.into())];
        let status = UpdatePresence::new(activities, false, None, status).unwrap();
        self.cluster.command(shard_id, &status).await?;

        Ok(())
    }
}

pub fn generate_activity(activity_type: ActivityType, message: String) -> Activity {
    Activity {
        assets: None,
        application_id: None,
        buttons: Vec::new(),
        created_at: None,
        details: None,
        flags: None,
        id: None,
        instance: None,
        kind: activity_type,
        name: message,
        emoji: None,
        party: None,
        secrets: None,
        state: None,
        timestamps: None,
        url: None,
    }
}
