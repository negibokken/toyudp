use std::net::UdpSocket;

const BUF_MAXSIZE: usize = 1500;

fn start_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("[::]:8080")?;
    let mut buf = [0; BUF_MAXSIZE];
    let (amt, src) = socket.recv_from(&mut buf)?;
    socket.connect(&src)?;
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    start_server()
}
