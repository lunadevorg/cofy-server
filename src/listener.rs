use std::{io::{BufReader, prelude::*}, net::TcpListener};

pub struct Listener {
    tcp: TcpListener
}

impl Listener {
    pub fn new(ip: &String, port: &i64) -> Self {
        let local_ip = format!("{}:{}", ip, port);
        Self {
            tcp: TcpListener::bind(local_ip).unwrap()
        }
    }

    pub fn handle_connections(&self) {
        for stream in self.tcp.incoming() {
            let stream = stream.unwrap();

            let reader = BufReader::new(stream);

            let http_request: Vec<_> = reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Request: {:#?}", http_request);
        }
    }
}