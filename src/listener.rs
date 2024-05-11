use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Interest},
    net::TcpListener,
};

type StringMap = HashMap<String, String>;
type Handler = fn(StringMap, String) -> StringMap;

pub struct Listener {
    listener: TcpListener,
    handlers: HashMap<String, Handler>,
}

impl Listener {
    pub async fn new(ip: String) -> Self {
        Self {
            listener: TcpListener::bind(ip)
                .await
                .expect("cannot intialize the listener on that address"),
            handlers: HashMap::new(),
        }
    }

    pub fn attach_handler(&mut self, path: String, handler: Handler) {
        self.handlers.insert(path, handler);
    }

    pub async fn main_loop(&mut self) {
        loop {
            let result = self.listener.accept().await;
            match result {
                Err(_) => continue,
                Ok(_) => (),
            };
            let (mut stream, addr) = result.unwrap();
            let result = stream.ready(Interest::READABLE | Interest::WRITABLE).await;

            match result {
                Err(_) => continue,
                Ok(_) => (),
            }
            let ready = result.unwrap();

            if ready.is_readable() {
                let mut buffer = vec![0; 1024];
                let _ = stream.read(&mut buffer);
                println!("data: {:#?}", buffer);
            }
        }
    }
}
