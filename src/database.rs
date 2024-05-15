/*
*     _____     ___
*    / ___/__  / _/_ __
*   / /__/ _ \/ _/ // /
*   \___/\___/_/ \_, /
*               /___/
*
*   database.rs: database manipulation using sqlx
*/

use crate::config::{Config, ServerModeration};
use anyhow::Result;
use sqlx::{pool::Pool, postgres::PgPoolOptions, Postgres};
use std::fs::read_to_string;

#[derive(Clone)]
pub struct Database {
    pub inside: Pool<Postgres>,
    #[allow(dead_code)]
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
            moderation: config.moderation,
        })
    }

    pub async fn run_script(&self, path: &String) -> Result<usize> {
        let query = read_to_string(path)?;
        sqlx::query(&query)
            .bind("0.0.0.0")
            .bind("self")
            .bind("0.0.1")
            .bind("/")
            .execute(&self.inside)
            .await?;
        Ok(0)
    }
}
