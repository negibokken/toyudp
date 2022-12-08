// use futures::channel::mpsc::Receiver;

use super::{connection::Connection, datagram::Datagram};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, mpsc::{Receiver, Sender}},
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
        let mut queue = self.recv_queue.lock().unwrap();
        let data= Datagram::new(
            (&mut recv_buf[..amt]).to_vec(),
            None,
            Some(src),
        );
        queue.push(data.clone());
        println!("received: {:?}", recv_buf);
        Some(data)
    }

    pub fn send_queue(&mut self, data: Datagram, dest: SocketAddr) {
        let mut queue = self.send_queue.lock().unwrap();
        queue.push(data.clone());

        println!("sent: {:?}", data);
        {
            let conn = self.conn.clone();
            conn.socket.send_to(&data.bytes, dest).unwrap();
        }
    }

    pub fn run(&self, receiver: Receiver<Datagram>, sender: Sender<Datagram>) -> JoinHandle<()> {
        let t1 = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(10));
            let _ = self.poll_recv_queue().unwrap();
            // let data = receiver.recv().unwrap();
            // if let Some(dest) = data.src() {
            //     println!("destination: {:?}", dest);
            //     let data = Datagram::new(b"hello world\n".to_vec(), Some(dest), None);
            //     // self.send_queue(data, dest);
            //     sender.send(data).unwrap();
            // }
        });
        let t2 = thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(10));
            let data = Datagram::new(b"hello world\n".to_vec(), Some(dest), None);
            if let Some(dest) = data.src() {
                self.send_queue(data, dest);
            }
        });
        t1
    }
}
