use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use crate::udp::datagram::Datagram;
use crate::udp::server;

pub fn connect(addr: &str) -> (Sender<Datagram>, Receiver<Datagram>) {
    let (sender,receiver): (Sender<Datagram>, Receiver<Datagram>) = mpsc::channel();
    let server = server::Server::new(addr, sender.clone());
    server.run();

    // thread::spawn(move || {
    //     loop {
    //         let data = receiver.recv().unwrap();
    //         println!("datagram: {:?}", data);
    //         if let Some(dest) = data.src() {
    //             thread::sleep(time::Duration::from_millis(10));
    //             let data = Datagram::new(b"hello\n".to_vec(), Some(dest), None);
    //             server.send(data);
    //         }
    //     }
    // });
    (sender,receiver)
}
