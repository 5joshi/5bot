mod methods;
mod models;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::error::BotResult;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> BotResult<Self> {
        let pool = PgPoolOptions::new().connect_lazy(url)?;
        Ok(Self { pool })
    }
}
