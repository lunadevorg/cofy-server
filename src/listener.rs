//use std::collections::HashMap;
use serde_json::to_string;
use std::{collections::HashMap, error::Error};
use tokio::net::TcpListener;

use crate::http_parse::{self, parse_http_request};

//type StringMap = HashMap<String, String>;
//type Handler = fn(StringMap, String) -> StringMap;

pub struct Listener {
    listener: TcpListener,
    //handlers: HashMap<String, Handler>,
}

impl Listener {
    pub async fn new(ip: String) -> Self {
        Self {
            listener: TcpListener::bind(ip)
                .await
                .expect("cannot intialize the listener on that address"),
            //handlers: HashMap::new(),
        }
    }

    //pub fn attach_handler(&mut self, path: String, handler: Handler) {
    //    self.handlers.insert(path, handler);
    //}

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let result = self.listener.accept().await;
            match result {
                Err(_) => continue,
                Ok(_) => (),
            };
            let (stream, addr) = result.unwrap();
            println!("{addr}");

            stream.readable().await?;
            let mut buffer = vec![0; 128];
            match stream.try_read(&mut buffer) {
                Ok(_) => (),
                Err(e) => println!("Error: {e}"),
            }
            let data = String::from_utf8_lossy(&buffer).into_owned();
            let result = parse_http_request(&data);

            match result {
                None => continue,
                Some(_) => (),
            }

            let (path, args) = result.unwrap();
            println!("{}", to_string(&args)?);
            let response = HashMap::from([("path".to_owned(), path)]);

            stream.writable().await?;
            let response = http_parse::construct_response(200, response);
            println!("{response}");

            let result = stream.try_write(response.as_bytes()).map_or(0, |x| x);
            assert_eq!(result, response.len());
        }
    }
}
