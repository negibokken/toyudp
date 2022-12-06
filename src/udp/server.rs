// use futures::channel::mpsc::Receiver;

use super::{connection::Connection, datagram::Datagram};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle}, time,
};

const BUF_MAXSIZE: usize = 1500;

pub struct Server {
    conn: Arc<Connection>,
    recv_queue: Mutex<Vec<Datagram>>,
    send_queue: Mutex<Vec<Datagram>>,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            conn: Arc::new(Connection::new(addr)),
            recv_queue: Mutex::new(vec![]),
            send_queue: Mutex::new(vec![]),
        }
    }

    fn poll_recv_queue(&mut self) -> Option<Datagram> {
        let conn = self.conn.clone();
        let mut recv_buf = [0; BUF_MAXSIZE];
        let (amt, src) = conn.socket.recv_from(&mut recv_buf).unwrap();
        println!("{:?}", recv_buf);
        Some(Datagram::new(
            (&mut recv_buf[..amt]).to_vec(),
            None,
            Some(src),
        ))
    }

    pub fn send_queue(&mut self, data: Datagram, dest: SocketAddr) {
        {
            let conn = self.conn.clone();
            let mut queue = self.send_queue.lock().unwrap();
            queue.push(data.clone());
            if let Some(dest) = data.dest {
                let _ = conn.socket.send_to(&data.bytes, dest);
            }
        }
    }

    pub fn run(mut self) -> JoinHandle<()> {
        let t = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(10));
            let data = self.poll_recv_queue().unwrap();
            if let Some(dest) = data.src {
                println!("here: {:?}", dest);
                let data = Datagram::new(b"hello world".to_vec(), Some(dest), None);
                self.send_queue(data, dest);
            }
        });
        t
    }
}