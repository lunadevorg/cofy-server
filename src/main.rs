mod config;
mod listener;

use std::path::Path;

fn main() {
    let config = config::Config::new(Path::new("cofy_config.toml"));
    println!("IP: {}:{}", config.ip, config.port);
    let listener = listener::Listener::new(&config.ip, config.port);

    listener.handle_connections();
}
