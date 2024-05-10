use anyhow::{Error, Result};
use std::{
    collections::HashMap, io::{prelude::*, BufReader}, net::TcpListener
};
use serde_json::to_string;

type Map = HashMap<String, String>;

pub struct Listener {
    tcp: TcpListener,
    handlers: HashMap<String, fn(Map) -> Map>
}

impl Listener {
    pub fn new(ip: &str, port: i64) -> Result<Self> {
        let new_ip = ip.replace("\"", "");
        let local_ip = format!("{new_ip}:{port}");
        Ok(Self {
            tcp: TcpListener::bind(local_ip)?,
            handlers: HashMap::new()
        })
    }

    pub fn attach_handler(&mut self, case: String, handler: fn(Map) -> Map) {
        self.handlers.insert(case, handler);
    }
    
    pub fn choose_handler(&self, path: &String) -> Option<&fn(Map) -> Map> {
        let handler = self.handlers.get(path.as_str());
        match handler {
            None => None,
            Some(_) => handler
        }
    }

    fn parse_http_request(request: &str) -> Option<(String, Map)> {
        let parts: Vec<&str> = request.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return None;
        }
    
        let (path_and_query, _version) = (parts[1], parts[0]);
    
        let mut path = String::new();
        let mut query_params = HashMap::new();
    
        if let Some(index) = path_and_query.find('?') {
            path = path_and_query[..index].to_string();
            let query = &path_and_query[index + 1..];
    
            for param in query.split('&') {
                let key_value: Vec<&str> = param.split('=').collect();
                if key_value.len() == 2 {
                    query_params.insert(key_value[0].to_string(), key_value[1].to_string());
                }
            }
        } else {
            path = path_and_query.split(' ').nth(0).unwrap().to_string();
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

            let res = Listener::parse_http_request(&http_request[0]);
            match res {
                None => return Err(Error::msg("couldn't parse the HTTP")),
                _ => ()
            }

            let (path, options) = res.unwrap();
            let handler = self.choose_handler(&path);

            let http_return_dict = match handler {
                None => Map::new(),
                _ => handler.unwrap()(options)
            };

            let http_return = to_string(&http_return_dict)?;

            let http_str = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{http_return}"
            );

            let _ = stream.write(http_str.as_bytes());

            println!("{path}");
        }
        Ok(())
    }
}
