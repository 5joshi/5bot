use chrono::{DateTime, Duration, Utc};
use futures::StreamExt;
use sqlx::Row;
use twilight_model::{channel::Message, id::ChannelId};

use crate::{commands::MessageActivity, database::Database, error::BotResult};

impl Database {
    pub async fn insert_message(&self, message: &Message) -> BotResult<bool> {
        let query = sqlx::query!(
            "INSERT INTO messages (id, channel_id, author, content, timestamp, bot) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING;",
            message.id.0 as i64,
            message.channel_id.0 as i64,
            message.author.id.0 as i64,
            message.content,
            message.timestamp.parse::<DateTime<Utc>>()?,
            message.author.bot
        );
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    /// Retrieve message counts from the past month, week, day and hour in that exact order.
    pub async fn get_activity(&self, channel_id: Option<ChannelId>) -> BotResult<MessageActivity> {
        let query = if let Some(id) = channel_id {
            sqlx::query("SELECT timestamp, bot FROM messages WHERE timestamp BETWEEN (now() - '1 month'::interval) and now() AND channel_id = $1")
                .bind(id.0 as i64)
        } else {
            sqlx::query("SELECT timestamp, bot FROM messages WHERE timestamp BETWEEN (now() - '1 month'::interval) and now()")
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
