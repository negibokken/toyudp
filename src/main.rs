use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use udp::datagram::Datagram;
use udp::server;

mod udp;

fn main() -> std::io::Result<()> {
    let server = server::Server::new("127.0.0.1:8080");
    let (sender1,receiver1): (Sender<Datagram>, Receiver<Datagram>) = mpsc::channel();
    let (sender2,receiver2): (Sender<Datagram>, Receiver<Datagram>) = mpsc::channel();
    server.run(receiver1, sender2).join().unwrap();
    let d = receiver2.recv().unwrap();
    println!("received: {:?}",d);
    if let Some(dest) = d.src() {
        let d = Datagram::new(b"helloworld".to_vec(), Some(dest), None);
        sender1.send(d).unwrap();
    }
    Ok(())
}
