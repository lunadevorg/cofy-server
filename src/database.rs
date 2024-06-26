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
use log::error;
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
        let query = read_to_string(path)?
            .replace("$1", "'0.0.0.0'")
            .replace("$2", "'self'")
            .replace("$3", "'0.0.1'")
            .replace("$4", "'/'");
        for line in query.lines() {
            if line
                .chars()
                .next()
                .is_some_and(|ch| ['/', '*', '\n'].contains(&ch))
            {
                continue;
            }
            let result = sqlx::query(line).execute(&self.inside).await;
            if let Err(err) = result {
                error!("{err}");
            }
        }

        Ok(0)
    }
}
