use anyhow::Result;
use std::{convert::From, fs::read_to_string, path::Path};
use toml::Table;

pub enum ServerModeration {
    Normal,
    None,
    Private,
    Unknown,
}

impl From<String> for ServerModeration {
    fn from(value: String) -> Self {
        match value.as_str() {
            "normal" => Self::Normal,
            "none" => Self::None,
            "private" => Self::Private,
            _ => Self::Unknown,
        }
    }
}

pub struct Config {
    pub ip: String,
    pub port: i64,
    pub moderation: ServerModeration,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self> {
        let buffer = read_to_string(path)?;

        let dict = buffer.parse::<Table>()?;

        Ok(Self {
            ip: dict["server"]["ip"].to_string(),
            port: dict["server"]["port"].as_integer().unwrap_or(8000),
            moderation: dict["server"]["moderation"].to_string().into(),
        })
    }
}
