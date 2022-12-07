use udp::server;

mod udp;

fn main() -> std::io::Result<()> {
    let server = server::Server::new("127.0.0.1:8080");
    server.run().join().unwrap();
    Ok(())
}
