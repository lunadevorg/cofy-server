use crate::http_parse::{parse_http_request, HandlerResult, StringMap};
use anyhow::{Error, Result};
use std::{
    borrow::ToOwned,
    collections::HashMap,
    future::Future,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct AsyncHandler<R>
where
    R: Future<Output = String>,
{
    pub func: fn(StringMap) -> R,
}

pub struct Listener {
    tcp: TcpListener,
    handlers: HashMap<String, AsyncHandler<HandlerResult>>,
}

impl Listener {
    pub fn new(ip: &str, port: i64) -> Result<Self> {
        let new_ip = format!("{ip}:{port}");
        Ok(Self {
            tcp: TcpListener::bind(new_ip)?,
            handlers: HashMap::new(),
        })
    }

    pub fn attach_handler(&mut self, case: String, handler: AsyncHandler<HandlerResult>) {
        self.handlers.insert(case, handler);
    }

    pub async fn choose_handler(&self, path: &str) -> &AsyncHandler<HandlerResult> {
        self.handlers.get(path).unwrap().to_owned()
    }

    pub async fn background_response(
        mut stream: &TcpStream,
        handler: &AsyncHandler<HandlerResult>,
        args: StringMap,
    ) {
        let data = (handler.func)(args).await;
        let _ = stream.write(data.as_bytes());
    }

    pub async fn handle_connections(&self) -> Result<()> {
        for stream in self.tcp.incoming() {
            let stream = stream?;

            let reader = BufReader::new(&stream);

            let http_request: Vec<_> = reader
                .lines()
                .map(|result| result.unwrap_or_else(|_| String::new()))
                .take_while(|line| !line.is_empty())
                .collect();

            let Some(res) = parse_http_request(&http_request[0]) else {
                return Err(Error::msg("couldn't parse the HTTP"));
            };

            let (path, options) = res;
            let handler = self.choose_handler(&path).await;

            Listener::background_response(&stream, handler, options).await;
        }
        Ok(())
    }
}
