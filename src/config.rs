use std::{io::Read, convert::From};
use toml::Table;

pub enum ServerModeration {
    ModerationNormal,
    ModerationNone,
    ModerationPrivate,
    ModerationUnknown
}

impl From<String> for ServerModeration {
    fn from(value: String) -> Self {
        match value.as_str() {
            "normal" => Self::ModerationNormal,
            "none" => Self::ModerationNone,
            "private" => Self::ModerationPrivate,
            _ => Self::ModerationUnknown
        }
    }
}

pub struct Config {
    pub ip: String,
    pub port: i64,
    pub moderation: ServerModeration
}

impl Config {
    pub fn new(path: &str) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer: String = String::new();
        let _ = file.read_to_string(&mut buffer);

        let dict = buffer.parse::<Table>().unwrap();

        Self {
            ip: dict["server"]["ip"].as_str().unwrap().to_string(),
            port: dict["server"]["port"].as_integer().unwrap_or(8000),
            moderation: dict["server"]["moderation"].to_string().into()
        }
    }
}