use crate::config::Config;
use anyhow::Result;
use sqlx::{pool::Pool, postgres::PgPoolOptions, Postgres};

pub struct Database {
    inside: Pool<Postgres>,
    db_path: String,
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
        })
    }

    pub async fn test(&self) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(format!("SELECT * FROM {};", self.db_path).as_str())
            .fetch_one(&self.inside)
            .await?;
        Ok(row.0)
    }
}
