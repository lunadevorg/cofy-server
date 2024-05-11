mod config;
mod http_parse;
mod listener;

use std::{error::Error, path::Path};

#[allow(clippy::print_stdout)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new(Path::new("cofy_config.toml"))?;
    let ip = format!("{}:{}", config.ip, config.port);
    println!("{}", &ip);

    let mut listener = listener::Listener::new(ip).await;
    listener.main_loop().await;

    Ok(())
}
