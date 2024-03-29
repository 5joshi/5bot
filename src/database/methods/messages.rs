use chrono::{DateTime, Duration, Utc};
use futures::StreamExt;
use regex::{escape, Regex};
use sqlx::Row;
use twilight_model::{
    channel::Message,
    id::{ChannelId, GuildId, UserId},
};

use crate::{commands::MessageActivity, database::Database, error::BotResult};

impl Database {
    pub async fn insert_message(&self, message: &Message) -> BotResult<bool> {
        let query = sqlx::query!(
            "INSERT INTO messages (id, guild_id, channel_id, author, content, timestamp, bot) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO NOTHING;",
            message.id.0 as i64,
            message.guild_id.map(|id| id.0 as i64),
            message.channel_id.0 as i64,
            message.author.id.0 as i64,
            message.content,
            message.timestamp.parse::<DateTime<Utc>>()?,
            message.author.bot
        );
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn get_messages(
        &self,
        author: Option<UserId>,
        channel: Option<ChannelId>,
        guild: GuildId,
    ) -> BotResult<Vec<String>> {
        let author = author.map(|a| a.0 as i64);
        let channel = channel.map(|a| a.0 as i64);
        let guild = guild.0 as i64;
        match (author, channel) {
            (Some(a), Some(c)) => {
                let mut stream = sqlx::query!(
                    "SELECT content FROM messages WHERE author = $1 AND channel_id = $2 AND content != ''",
                    a,
                    c
                )
                .fetch(&self.pool);
                let mut messages = Vec::new();
                while let Some(entry) = stream.next().await.transpose()? {
                    messages.push(entry.content)
                }
                Ok(messages)
            }
            (Some(a), None) => {
                let mut stream = sqlx::query!(
                    "SELECT content FROM messages WHERE author = $1 AND content != ''",
                    a
                )
                .fetch(&self.pool);
                let mut messages = Vec::new();
                while let Some(entry) = stream.next().await.transpose()? {
                    messages.push(entry.content)
                }
                Ok(messages)
            }
            (None, Some(c)) => {
                let mut stream = sqlx::query!(
                    "SELECT content FROM messages WHERE channel_id = $1 AND bot = false AND content != ''",
                    c
                )
                .fetch(&self.pool);
                let mut messages = Vec::new();
                while let Some(entry) = stream.next().await.transpose()? {
                    messages.push(entry.content)
                }
                Ok(messages)
            }
            (None, None) => {
                let mut stream = sqlx::query!(
                    "SELECT content FROM messages WHERE guild_id = $1 AND bot = false AND content != ''",
                    guild
                )
                .fetch(&self.pool);
                let mut messages = Vec::new();
                while let Some(entry) = stream.next().await.transpose()? {
                    messages.push(entry.content)
                }
                Ok(messages)
            }
        }
    }

    pub async fn get_complete_messages(
        &self,
        author: Option<UserId>,
        channel: Option<ChannelId>,
        contains: &str,
        guild: GuildId,
    ) -> BotResult<Vec<String>> {
        let re = Regex::new(&format!(r".*{}.*", escape(contains)))?;
        let mut messages = self.get_messages(author, channel, guild).await?;
        messages.retain(|m| re.is_match(m));
        Ok(messages)
    }

    pub async fn get_regex_messages(
        &self,
        author: Option<UserId>,
        channel: Option<ChannelId>,
        regex: &str,
        guild: GuildId,
    ) -> BotResult<Vec<String>> {
        let re = Regex::new(&format!(r"{}", regex))?;
        let mut messages = self.get_messages(author, channel, guild).await?;
        messages.retain(|m| re.is_match(m));
        Ok(messages)
    }

    /// Retrieve message counts from the past month, week, day and hour in that exact order.
    pub async fn get_activity(
        &self,
        guild_id: GuildId,
        channel_id: Option<ChannelId>,
    ) -> BotResult<MessageActivity> {
        let query = if let Some(id) = channel_id {
            sqlx::query("SELECT timestamp, bot FROM messages WHERE timestamp BETWEEN (now() - '1 month'::interval) and now() AND channel_id = $1 AND guild_id = $2")
                .bind(id.0 as i64).bind(guild_id.0 as i64)
        } else {
            sqlx::query("SELECT timestamp, bot FROM messages WHERE timestamp BETWEEN (now() - '1 month'::interval) and now() AND guild_id = $1")
                .bind(guild_id.0 as i64)
        };
        let mut stream = query.fetch(&self.pool);
        let curr_time = Utc::now();
        let mut counts = MessageActivity::default();
        while let Some(row) = stream.next().await.transpose()? {
            let (bot, timestamp) = (row.get("bot"), row.get::<DateTime<Utc>, _>("timestamp"));
            if bot {
                continue;
            }
            counts.month += 1;
            if timestamp > curr_time - Duration::weeks(1) {
                counts.week += 1;
                if timestamp > curr_time - Duration::days(1) {
                    counts.day += 1;
                    if timestamp > curr_time - Duration::hours(1) {
                        counts.hour += 1;
                    }
                }
            }
        }
        Ok(counts)
    }
}
