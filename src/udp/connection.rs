use std::{sync::Arc, net::UdpSocket};

pub struct Connection {
    pub socket: Arc<UdpSocket>,
}


impl Connection {
   pub fn new (s: &str) -> Connection {
    let conn = UdpSocket::bind(s).unwrap();
    Connection { socket: Arc::new(conn) }
   }
}
