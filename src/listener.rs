use anyhow::{Error, Result};
use http::StatusCode;
use serde_json::to_string;
use std::{
    borrow::ToOwned,
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpListener,
};

type Map = HashMap<String, String>;

pub struct HandlerResult {
    pub code: u16,
    pub detail: String,
    pub result: Map,
}

impl HandlerResult {
    pub fn new() -> Self {
        Self {
            code: 404,
            detail: "not found".to_owned(),
            result: Map::new(),
        }
    }
}

fn default_handler(_: Map) -> HandlerResult {
    HandlerResult::new()
}

pub struct Listener {
    tcp: TcpListener,
    handlers: HashMap<String, fn(Map) -> HandlerResult>,
}

impl Listener {
    pub fn new(ip: &str, port: i64) -> Result<Self> {
        let new_ip = format!("{ip}:{port}");
        Ok(Self {
            tcp: TcpListener::bind(new_ip)?,
            handlers: HashMap::new(),
        })
    }

    pub fn attach_handler(&mut self, case: String, handler: fn(Map) -> HandlerResult) {
        self.handlers.insert(case, handler);
    }

    pub fn choose_handler(&self, path: &str) -> fn(Map) -> HandlerResult {
        self.handlers
            .get(path)
            .map_or(default_handler, ToOwned::to_owned)
    }

    fn parse_http_request(request: &str) -> Option<(String, Map)> {
        let parts: Vec<&str> = request.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return None;
        }

        let (path_and_query, _version) = (parts[1].split_once(' ')?.0, parts[0]);

        let path: String;
        let mut query_params = HashMap::new();

        if let Some(index) = path_and_query.find('?') {
            path = path_and_query.get(..index)?.to_owned();
            let query = &path_and_query.get(index + 1..)?;

            for param in query.split('&') {
                let key_value: Vec<&str> = param.split('=').collect();
                if key_value.len() == 2 {
                    query_params.insert(key_value[0].to_owned(), key_value[1].to_owned());
                }
            }
        } else {
            path = path_and_query.to_owned();
        }

        Some((path, query_params))
    }

    pub fn handle_connections(&self) -> Result<()> {
        for stream in self.tcp.incoming() {
            let mut stream = stream?;

            let reader = BufReader::new(&stream);

            let http_request: Vec<_> = reader
                .lines()
                .map(|result| result.unwrap_or_else(|_| String::new()))
                .take_while(|line| !line.is_empty())
                .collect();

            let Some(res) = Self::parse_http_request(&http_request[0]) else {
                return Err(Error::msg("couldn't parse the HTTP"));
            };

            let (path, options) = res;
            let handler = self.choose_handler(&path);

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
