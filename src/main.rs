use udp::server;

mod udp;

fn main() -> std::io::Result<()> {
    let mut server = server::Server::new("127.0.0.1:8080");
    let t = server.run();
    t.join();
    Ok(())
}
