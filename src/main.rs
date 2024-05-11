mod config;
mod http_parse;
mod listener;

use anyhow::Result;
use http_parse::HandlerResult;
use std::{collections::HashMap, path::Path};

fn test_handler(map: HashMap<String, String>) -> HandlerResult {
    if map.contains_key("foo") {
        HandlerResult {
            code: 200,
            result: HashMap::from([
                ("which foo".to_owned(), map["foo"].clone()),
                ("detail".to_owned(), "works".to_owned()),
            ]),
        }
    } else {
        HandlerResult {
            code: 200,
            result: HashMap::from([("detail".to_owned(), "no foo".to_owned())]),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    println!("IP: {}:{}", config.ip, config.port);
    println!("Moderation: {}", String::from(config.moderation));

    let mut listener = listener::Listener::new(&config.ip, config.port)?;
    listener.attach_handler(
        "/".to_owned(),
        listener::AsyncHandler {
            func: (test_handler),
        },
    );

    listener.handle_connections().await?;
    Ok(())
}
