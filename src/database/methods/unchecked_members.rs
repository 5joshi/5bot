use crate::{database::Database, error::BotResult};

use chrono::{DateTime, Utc};
use futures::StreamExt;
use hashbrown::HashMap;
use twilight_model::id::UserId;

impl Database {
    pub async fn get_unchecked_members(&self) -> BotResult<HashMap<UserId, DateTime<Utc>>> {
        let mut stream = sqlx::query!("SELECT * FROM unchecked_members;").fetch(&self.pool);
        let mut unchecked_members = HashMap::new();
        while let Some(entry) = stream.next().await.transpose()? {
            let user_id = UserId(entry.user_id as u64);
            let joined = entry.joined;
            unchecked_members.insert(user_id, joined);
        }
        Ok(unchecked_members)
    }

    pub async fn insert_unchecked_member(&self, user_id: UserId) -> BotResult<bool> {
        let query = sqlx::query!(
            "INSERT INTO unchecked_members (user_id) VALUES ($1) ON CONFLICT (user_id) DO UPDATE SET joined = CURRENT_TIMESTAMP;",
            user_id.0 as i64
        );
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn remove_unchecked_member(&self, user_id: UserId) -> BotResult<bool> {
        let query = sqlx::query!(
            "DELETE FROM unchecked_members WHERE user_id = $1;",
            user_id.0 as i64
        );
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }
}
