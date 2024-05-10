mod listener;
mod config;

fn main() -> () {
    let config = config::Config::new("cofy_config.toml");
    let listener = listener::Listener::new(config.ip, config.port);

    
}