mod config;
mod http_parse;
mod listener;

use anyhow::{Context, Result};
use log::info;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))
        .with_context(|| "Failed to load config file")?;
    let ip = format!("{}:{}", config.ip, config.port);
    info!("{ip}");

    let mut listener = listener::Listener::new(ip)
        .await
        .with_context(|| "Failed to create listener")?;

    listener
        .main_loop()
        .await
        .with_context(|| "Failed to run loop")?;

    Ok(())
}
