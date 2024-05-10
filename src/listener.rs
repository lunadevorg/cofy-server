use std::net::TcpListener;

pub struct Listener {
    tcp: TcpListener
}

impl Listener {
    pub fn new(ip: String, port: i64) -> Self {
        let local_ip = format!("{}:{}", ip, port);
        Self {
            tcp: TcpListener::bind(local_ip).unwrap()
        }
    }
}