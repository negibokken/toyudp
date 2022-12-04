use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await { eprint!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum StreamState {
    Ready,
    Send,
    DataSent,
    ResetSent,
    DataRecvd,
    ResetRecvd,
}

struct Frame {
    typ: u8,
    field: u8,
}

struct Payload {
    frame: Frame,
}

struct Packet{
    number: u64,
    payload: Payload,
}


#[cfg(test)]
mod tests {
    use crate::StreamState;

    fn foobar() {
        let s = StreamState::DataRecvd;
        println!("{:?}",s);
    }
}
