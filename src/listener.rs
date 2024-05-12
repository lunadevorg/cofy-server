//use std::collections::HashMap;
use anyhow::Result;
use log::{error, info};
use serde_json::to_string;
use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::http_parse::{self, parse_http_request};

//type StringMap = HashMap<String, String>;
//type Handler = fn(StringMap, String) -> StringMap;

pub struct Listener {
    listener: TcpListener,
    //handlers: HashMap<String, Handler>,
}

impl Listener {
    pub async fn new(ip: String) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(ip).await?,
            //handlers: HashMap::new(),
        })
    }

    //pub fn attach_handler(&mut self, path: String, handler: Handler) {
    //    self.handlers.insert(path, handler);
    //}

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
            info!("{}", to_string(&args)?);
            let response = HashMap::from([("path".to_owned(), path)]);

            stream.writable().await?;
            let response = http_parse::construct_response(200, &response);
            info!("{response}");

            let result = stream.try_write(response.as_bytes()).unwrap_or_default();
            assert_eq!(
                result,
                response.len(),
                "the data written in stream must have the same length as the response"
            );
        }
    }
}
