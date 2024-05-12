//use std::collections::HashMap;
use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use tokio::net::TcpListener;

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

    // remove false-positive lint
    #[allow(clippy::unused_async)]
    async fn handle_request(_path: String, args: StringMap) -> String {
        http_parse::construct_response(200, &args)
    }

    pub async fn main_loop(&mut self) -> Result<()> {
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

            let (path, args) = parse_http_request(&data).unwrap_or_default();
            let result = tokio::spawn(Self::handle_request(path, args)).await?;
            let size = stream.try_write(result.as_bytes()).unwrap_or_default();
            assert_eq!(
                size,
                result.len(),
                "the data written in stream must have the same length as the response"
            );
        }
    }
}
