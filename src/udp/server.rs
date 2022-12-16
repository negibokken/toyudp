// use futures::channel::mpsc::Receiver;

use super::{connection::Connection, datagram::Datagram};
use std::{
    sync::{Arc, mpsc::Sender},
    thread,
    thread::JoinHandle,
    time
};

const BUF_MAXSIZE: usize = 1500;

pub struct Server {
    pub conn: Arc<Connection>,
    sender: Sender<Datagram>,
}

impl Server {
    pub fn new(addr: &str, sender: Sender<Datagram>) -> Server {
        Server {
            conn: Arc::new(Connection::new(addr)),
            sender,
        }
    }

    pub fn send(&self, data: Datagram) {
        let socket = self.conn.socket.clone();
        match data.dest() {
        Some(dest) => {
            socket.send_to(&data.bytes,dest).unwrap();
        }
        None => {
            println!("hellow");
        }
       }

    }

    pub fn run(&self) -> JoinHandle<()> {
        let socket = self.conn.socket.clone();
        let sender = self.sender.clone();

        let mut buf =  [0; BUF_MAXSIZE];

        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(10));
            let (_, src) = socket.recv_from(&mut buf).unwrap();
            sender.send(Datagram::new(buf.to_vec(), None, Some(src))).unwrap();
        })
    }
}
