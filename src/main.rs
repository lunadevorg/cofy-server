mod config;
mod listener;

use anyhow::Result;
use std::{collections::HashMap, path::Path};

fn test_handler(_map: HashMap<String, String>) -> HashMap<String, String> {
    HashMap::from([
        ("hello".to_string(), "world".to_string())
    ])
}

fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    println!("IP: {}:{}", config.ip.replace("\"", ""), config.port);
    let mut listener = listener::Listener::new(&config.ip, config.port)?;
    listener.attach_handler("/".to_string(), test_handler);

    listener.handle_connections()?;
    Ok(())
}
