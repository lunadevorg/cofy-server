mod config;
mod http_parse;
mod listener;

use anyhow::Result;
use log::info;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    let ip = format!("{}:{}", config.ip, config.port);
    info!("{ip}");

    let mut listener = listener::Listener::new(ip).await?;
    listener.main_loop().await?;

    Ok(())
}
