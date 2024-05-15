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

    async fn handler(
        db: Database,
        state: ServerModeration,
        stream: TcpStream,
        args: StringMap,
    ) -> usize {
        let response = if args.contains_key("moder") {
            http_parse::new_str_response(200, &format!("{{\"moder\" : {}}}", String::from(state)))
        } else if args.contains_key("test") {
            http_parse::new_str_response(
                200,
                &format!("{{\"test\" : {}}}", db.test().await.unwrap_or_default()),
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
            loop {
                let result = stream.try_read(&mut buffer).unwrap_or_default();
                let data = String::from_utf8_lossy(&buffer).into_owned();
                to_parse += &data;
                if result != 320 && !first {
                    break;
                } else if first {
                    first = false;
                }
            }

            let (_path, args) = parse_http_request(&to_parse).unwrap_or_default();
            let result = tokio::spawn(Self::handler(
                self.database.clone(),
                self.database.moderation.clone(),
                stream,
                args,
            ));
            assert_ne!(
                result.await.unwrap_or_default(),
                0,
                "Couldn't write message into the stream"
            );
        }
    }
}
