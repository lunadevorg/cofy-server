use crate::http_parse::{parse_http_request, HandlerResult, StringMap};
use anyhow::{Error, Result};
use http::StatusCode;
use serde_json::to_string;
use std::{
    borrow::ToOwned,
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpListener,
};

fn default_handler(_: StringMap) -> HandlerResult {
    HandlerResult::new()
}

type Handler = fn(StringMap) -> HandlerResult;

pub struct Listener {
    tcp: TcpListener,
    handlers: HashMap<String, Handler>,
}

impl Listener {
    pub fn new(ip: &str, port: i64) -> Result<Self> {
        let new_ip = format!("{ip}:{port}");
        Ok(Self {
            tcp: TcpListener::bind(new_ip)?,
            handlers: HashMap::new(),
        })
    }

    pub fn attach_handler(&mut self, case: String, handler: Handler) {
        self.handlers.insert(case, handler);
    }

    pub async fn choose_handler(&self, path: &str) -> Handler {
        self.handlers
            .get(path)
            .map_or(default_handler, ToOwned::to_owned)
    }

    pub async fn handle_connections(&self) -> Result<()> {
        for stream in self.tcp.incoming() {
            let mut stream = stream?;

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

            let handler_result = handler(options);
            let mut http_return_dict = handler_result.result;
            http_return_dict.insert("detail".to_owned(), handler_result.detail);

            let http_return = to_string(&http_return_dict)?;

            let return_code = handler_result.code;
            let status_code = StatusCode::from_u16(handler_result.code)?.to_string();

            let http_str = format!(
                "HTTP/1.1 {return_code} {status_code}\r\nContent-Type: application/json\r\n\r\n{http_return}"
            );

            stream.write_all(http_str.as_bytes())?;
        }
        Ok(())
    }
}
