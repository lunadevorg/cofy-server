use anyhow::Result;
use std::{convert::From, fs::read_to_string, path::Path};
use toml::Table;

#[derive(Clone)]
pub enum ServerModeration {
    Normal,
    None,
    Private,
}

impl From<String> for ServerModeration {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => Self::None,
            "private" => Self::Private,
            _ => Self::Normal,
        }
    }
}

impl From<ServerModeration> for String {
    fn from(value: ServerModeration) -> Self {
        match value {
            ServerModeration::None => "\"none\"".to_owned(),
            ServerModeration::Normal => "\"normal\"".to_owned(),
            ServerModeration::Private => "\"private\"".to_owned(),
        }
    }
}

pub struct Config {
    pub ip: String,
    pub port: i64,
    #[allow(dead_code)] //Moderation will be used when handlers are implemented
    pub moderation: ServerModeration,
    pub db_path: String,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self> {
        let buffer = read_to_string(path)?;

        let dict = buffer.parse::<Table>()?;

        Ok(Self {
            ip: dict["server"]["ip"].to_string().replace('"', ""),
            port: dict["server"]["port"].as_integer().unwrap_or(8000),
            moderation: dict["server"]["moderation"]
                .to_string()
                .replace('"', "")
                .into(),
            db_path: dict["db"]["path"].to_string(),
            db_user: dict["db"]["user"].to_string().replace('"', ""),
            db_password: dict["db"]["password"].to_string().replace('"', ""),
        })
    }
}
