use crate::config::{Config, ServerModeration};
use anyhow::Result;
use sqlx::{pool::Pool, postgres::PgPoolOptions, Postgres};

#[derive(Clone)]
pub struct Database {
    pub inside: Pool<Postgres>,
    #[allow(dead_code)]
    pub db_path: String,
    pub moderation: ServerModeration,
}

impl Database {
    pub async fn new(config: Config) -> Result<Self> {
        //Database must always have the name cofy_server
        let path = format!(
            "postgresql://localhost/cofy_server?user={}&password={}",
            config.db_user, config.db_password
        );
        let db = PgPoolOptions::new().connect(&path).await?;
        Ok(Self {
            inside: db,
            db_path: config.db_path,
            moderation: config.moderation,
        })
    }

    pub async fn test(&self) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(&format!("SELECT * FROM {};", self.db_path))
            .fetch_one(&self.inside)
            .await?;
        Ok(row.0)
    }
}
