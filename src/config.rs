/*
*     _____     ___
*    / ___/__  / _/_ __
*   / /__/ _ \/ _/ // /
*   \___/\___/_/ \_, /
*               /___/
*
*   config.rs: parse the cofy_config.toml file to get server configuration
*   parameters (in cofy_config.toml):
*       server.ip: the ip address of the server (string)
*       server.port: server's port (u16)
*       server.moderation: default moderation level of the server
*       db.user: the user for your postgres database
*       db.password: user's password
*/

use anyhow::Result;
use std::{
    convert::From,
    fmt::{self, Display},
    fs::read_to_string,
    path::Path,
};
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

impl Display for ServerModeration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::None => write!(f, "\"none\""),
            Self::Normal => write!(f, "\"normal\""),
            Self::Private => write!(f, "\"private\""),
        }
    }
}

pub struct Config {
    pub ip: String,
    pub port: i64,
    pub moderation: ServerModeration,
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
            db_user: dict["db"]["user"].to_string().replace('"', ""),
            db_password: dict["db"]["password"].to_string().replace('"', ""),
        })
    }
}
