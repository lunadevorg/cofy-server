use anyhow::Result;
use log::info;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

use crate::config::ServerModeration;
use crate::database::Database;
use crate::http_parse::{self, parse_http_request};

pub type StringMap = HashMap<String, String>;

pub struct Listener {
    listener: TcpListener,
    database: Database,
}

impl Listener {
    pub async fn new(ip: String, db: Database) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(ip).await?,
            database: db,
        })
    }

    #[allow(clippy::unused_async)] //Magic tool we'll put to good use later
    async fn handler(state: ServerModeration, stream: TcpStream, args: StringMap) -> usize {
        let response = if args.contains_key("moder") {
            http_parse::new_str_response(
                200,
                format!("{{\"moder\" : {}}}", String::from(state)).as_str(),
            )
        } else {
            http_parse::new_response(200, &args)
        };
        stream.try_write(response.as_bytes()).unwrap_or_default()
    }

    pub async fn main_loop(&self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            info!("{addr}");

            stream.readable().await?;
            let mut buffer = vec![0; 320];
            let mut to_parse = String::new();

            let mut first = true;
            while stream.try_read(&mut buffer).unwrap_or_default() == 320 || first {
                let data = String::from_utf8_lossy(&buffer).into_owned();
                to_parse += &data;
                first = false;
            }

            let (_path, args) = parse_http_request(&to_parse).unwrap_or_default();
            let result = tokio::spawn(Self::handler(
                self.database.moderation.clone(),
                stream,
                args,
            ));
            assert!(
                result.await.unwrap_or_default() != 0,
                "Couldn't write message into the stream"
            );
        }
    }
}
