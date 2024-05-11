//use std::collections::HashMap;
use std::error::Error;
use tokio::net::TcpListener;

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
            println!("{}", String::from_utf8_lossy(&buffer));
        }
    }
}
