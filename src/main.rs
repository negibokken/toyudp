use udp::connection;

mod udp;

const BUF_MAXSIZE: usize = 1500;

fn main() -> std::io::Result<()> {
    let conn = connection::Connection::new("127.0.0.1:8080");
    loop {
        let sock = conn.socket.clone();
        let mut recv_buf = [0; BUF_MAXSIZE];
        let (amt, src) = sock.recv_from(&mut recv_buf)?;
        let recv_buf = &mut recv_buf[..amt];
        println!("{:?}", recv_buf);
        let buf = b"hello world\n";
        sock.send_to(buf, src).unwrap();
    }
}
