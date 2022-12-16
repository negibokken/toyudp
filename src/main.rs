mod udp;
mod quic;
use quic::{udp_connector};


fn main() -> std::io::Result<()> {
    udp_connector::connect("127.0.0.1:8080");
    Ok(())
}
