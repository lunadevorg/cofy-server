mod config;
mod listener;

use anyhow::Result;
use listener::HandlerResult;
use std::{collections::HashMap, path::Path};

fn test_handler(_map: HashMap<String, String>) -> HandlerResult {
    HandlerResult {
        code: 200,
        detail: "success".to_string(),
        result: HashMap::new()
    }
}

fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    println!("IP: {}:{}", config.ip, config.port);
    println!("Moderation: {}", String::from(config.moderation));
    let mut listener = listener::Listener::new(&config.ip, config.port)?;
    listener.attach_handler("/".to_string(), test_handler);

    listener.handle_connections()?;
    Ok(())
}
