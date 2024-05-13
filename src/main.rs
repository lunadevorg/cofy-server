mod config;
mod database;
mod http_parse;
mod listener;

use anyhow::{Context, Result};
use database::Database;
use log::info;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))
        .with_context(|| "Failed to load config file")?;
    let ip = format!("{}:{}", config.ip, config.port);
    info!("{ip}");

    let database = Database::new(config)
        .await
        .with_context(|| "Failed to create the database")?;

    let listener = listener::Listener::new(ip, database)
        .await
        .with_context(|| "Failed to create the listener")?;

    listener
        .main_loop()
        .await
        .with_context(|| "Failed to run loop")?;

    Ok(())
}
