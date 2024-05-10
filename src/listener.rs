use anyhow::Result;
use std::{
    io::{prelude::*, BufReader},
    net::TcpListener,
};

pub struct Listener {
    tcp: TcpListener,
}

impl Listener {
    pub fn new(ip: &str, port: i64) -> Result<Self> {
        let local_ip = format!("{ip}:{port}");
        Ok(Self {
            tcp: TcpListener::bind(local_ip)?,
        })
    }

    pub fn handle_connections(&self) -> Result<()> {
        for stream in self.tcp.incoming() {
            let stream = stream?;

            let reader = BufReader::new(stream);

            let http_request: Vec<_> = reader
                .lines()
                .map(|result| result.unwrap_or_else(|_| String::new()))
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Request: {http_request:#?}");
        }
        Ok(())
    }
}
