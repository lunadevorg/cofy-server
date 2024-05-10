mod config;
mod listener;

use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    println!("IP: {}:{}", config.ip, config.port);
    let listener = listener::Listener::new(&config.ip, config.port);

    listener?.handle_connections()?;
    Ok(())
}
