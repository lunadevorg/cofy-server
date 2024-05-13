use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

use crate::http_parse::{self, parse_http_request};

pub type StringMap = HashMap<String, String>;

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub async fn new(ip: String) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(ip).await?,
        })
    }

    #[allow(clippy::unused_async)] //Magic tool we'll put to good use later
    async fn handler(stream: TcpStream, args: StringMap) -> usize {
        let response = http_parse::construct_response(200, &args);
        stream.try_write(response.as_bytes()).unwrap_or_default()
    }

    pub async fn main_loop(&self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            info!("{addr}");

            stream.readable().await?;
            //320 elements to allow reading all data
            let mut buffer = vec![0; 320];
            match stream.try_read(&mut buffer) {
                Ok(_) => (),
                Err(err) => error!("{err}"),
            }
            let data = String::from_utf8_lossy(&buffer).into_owned();

            let (_path, args) = parse_http_request(&data).unwrap_or_default();
            let result = tokio::spawn(Self::handler(stream, args));
            assert!(
                result.await.unwrap_or_default() != 0,
                "Couldn't write message into the stream"
            );
        }
    }
}
