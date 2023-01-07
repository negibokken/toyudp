use hex_literal::hex;
use hkdf::Hkdf;
use sha2::Sha256;
use std::{net::UdpSocket, thread, sync::Arc};

fn expand_label(label: &str, length: usize) -> Vec<u8> {
    let label = format!("tls13 {}", label);
    let label_len = label.len();
    let res = [
        &[0],
        &[length as u8], // padding for length [0] and length
        &[label_len as u8],
        &label.as_bytes()[..],
        &[0], // context length
    ]
    .concat();
    return res;
}

fn get_initial_keys(
    initial_salt: &[u8],
    initial_random: &[u8],
    is_client: bool,
) -> ([u8; 16], [u8; 12], [u8; 16]) {
    let (_, hk) = Hkdf::<Sha256>::extract(Some(&initial_salt), &initial_random);

    let label = if is_client {
        expand_label("client in", 32)
    } else {
        expand_label("server in", 32)
    };

    let mut secret = [0u8; 32];
    hk.expand(label.as_slice(), &mut secret).unwrap();

    let hk = Hkdf::<Sha256>::from_prk(&secret).unwrap();
    let mut key = [0u8; 16];
    hk.expand(expand_label("quic key", 16).as_slice(), &mut key)
        .unwrap();

    let hk = Hkdf::<Sha256>::from_prk(&secret).unwrap();
    let mut iv = [0u8; 12];
    hk.expand(expand_label("quic iv", 12).as_slice(), &mut iv)
        .unwrap();

    let mut hp_key = [0u8; 16];
    hk.expand(expand_label("quic hp", 16).as_slice(), &mut hp_key)
        .unwrap();

    return (key, iv, hp_key);
}

fn get_initial_packet() -> Vec<u8> {
// Client Initial Packet
    // Packet Header Byte
    let packet_header_byte: [u8; 1] = [205];

    // QUIC Version
    let version: [u8; 4] = [0, 0, 0, 1];

    let destination_connection_id: [u8; 9] = [8, 0, 1, 2, 3, 4, 5, 6, 7];

    let source_connection_id: [u8; 6] = [5, 99, 95, 99, 105, 100];

    let token: [u8; 1] = [0];

    let packet_length: [u8; 2] = [65, 3];

    // encrypted packet number (00)
    let packet_number: [u8; 1] = [152];

    let encrypted_data =  hex!("
        1c 36 a7 ed 78 71 6b e9 71 1b a4 98 b7 ed 86 84 43 bb 2e 0c 51 4d 4d 84 8e ad cc 7a 00 d2 5c e9 f9 af a4 83 97 80 88 de 83 6b e6 8c 0b 32 a2 45 95 d7 81 3e a5 41 4a 91 99 32 9a 6d 9f 7f 76 0d d8 bb 24 9b f3 f5 3d 9a 77 fb b7 b3 95 b8 d6 6d 78 79 a5 1f e5 9e f9 60 1f 79 99 8e b3 56 8e 1f dc 78 9f 64 0a ca b3 85 8a 82 ef 29 30 fa 5c e1 4b 5b 9e a0 bd b2 9f 45 72 da 85 aa 3d ef 39 b7 ef af ff a0 74 b9 26 70 70 d5 0b 5d 07 84 2e 49 bb a3 bc 78 7f f2 95 d6 ae 3b 51 43 05 f1 02 af e5 a0 47 b3 fb 4c 99 eb 92 a2 74 d2 44 d6 04 92 c0 e2 e6 e2 12 ce f0 f9 e3 f6 2e fd 09 55 e7 1c 76 8a a6 bb 3c d8 0b bb 37 55 c8 b7 eb ee 32 71 2f 40 f2 24 51 19 48 70 21 b4 b8 4e 15 65 e3 ca 31 96 7a c8 60 4d 40 32 17 0d ec 28 0a ee fa 09 5d 08
    ");

    let auth_tag = hex!(
        "
        b3 b7 24 1e f6 64 6a 6c 86 e5 c6 2c e0 8b e0 99
    "
    );

    let crypto_frame_header = hex!(
        "
        06 00 40 ee
    "
    );

    let client_hello = hex!("
        01 00 00 ea 03 03 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f 00 00 06 13 01 13 02 13 03 01 00 00 bb 00 00 00 18 00 16 00 00 13 65 78 61 6d 70 6c 65 2e 75 6c 66 68 65 69 6d 2e 6e 65 74 00 0a 00 08 00 06 00 1d 00 17 00 18 00 10 00 0b 00 09 08 70 69 6e 67 2f 31 2e 30 00 0d 00 14 00 12 04 03 08 04 04 01 05 03 08 05 05 01 08 06 06 01 02 01 00 33 00 26 00 24 00 1d 00 20 35 80 72 d6 36 58 80 d1 ae ea 32 9a df 91 21 38 38 51 ed 21 a2 8e 3b 75 e9 65 d0 d2 cd 16 62 54 00 2d 00 02 01 01 00 2b 00 03 02 03 04 00 39 00 31 03 04 80 00 ff f7 04 04 80 a0 00 00 05 04 80 10 00 00 06 04 80 10 00 00 07 04 80 10 00 00 08 01 0a 09 01 0a 0a 01 03 0b 01 19 0f 05 63 5f 63 69 64
    ");

    let content = [
        &packet_header_byte[..],
        &version[..],
        &destination_connection_id[..],
        &source_connection_id[..],
        &token[..],
        &packet_length[..],
        &packet_number[..],
        &encrypted_data[..],
        &auth_tag[..],
        &crypto_frame_header[..],
        &client_hello[..],
    ]
    .concat();

    let vec: Vec<u8> = vec![0; 1200 - content.len()];
    let content = [&content[..], &vec[..]].concat();
    return content;
}

fn run_server() -> Arc<UdpSocket> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("UdpSocket can't be binded");
    let socket_arc = Arc::new(socket);

    let cloned_socket = socket_arc.clone();

    thread::spawn(move || {
        let mut buf: [u8; 1500] = [0; 1500];
        loop {
            let (size, soc) = cloned_socket.recv_from(&mut buf).unwrap();
            println!("{:?}", std::str::from_utf8(&buf[0..size]).unwrap());
            cloned_socket.send_to(b"pong", soc).unwrap();
            cloned_socket.send_to(b"pong", soc).unwrap();
        }
    });
    socket_arc
}

fn main() {
    // privkey: 202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f
    // pubkey: 358072d6365880d1aeea329adf9121383851ed21a28e3b75e965d0d2cd166254

    // Client Initial Keys Calc
    let initial_random = hex!("0001020304050607");
    let initial_salt = hex!("38762cf7f55934b34d179ae6a4c80cadccbb7f0a");

    let (client_key, client_iv, client_hp) =
        get_initial_keys(&initial_salt, &initial_random, false);
    let (server_key, server_iv, server_hp) = get_initial_keys(&initial_salt, &initial_random, true);

}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;
    use std::thread;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::sync::Arc;

    use crate::get_initial_keys;
    use crate::run_server;
    use hex_literal::hex;



    #[test]
    fn test_get_client() {
        let initial_random = hex!("0001020304050607");
        let initial_salt = hex!("38762cf7f55934b34d179ae6a4c80cadccbb7f0a");

        let (client_key, client_iv, client_hp_key) =
            get_initial_keys(&initial_salt, &initial_random, true);
        let (server_key, server_iv, server_hp_key) =
            get_initial_keys(&initial_salt, &initial_random, false);

        assert_eq!(client_key[..], hex!("b14b918124fda5c8d79847602fa3520b"));
        assert_eq!(client_iv[..], hex!("ddbc15dea80925a55686a7df"));
        assert_eq!(client_hp_key[..], hex!("6df4e9d737cdf714711d7c617ee82981"));

        assert_eq!(server_key[..], hex!("d77fc4056fcfa32bd1302469ee6ebf90"));
        assert_eq!(server_iv[..], hex!("fcb748e37ff79860faa07477"));
        assert_eq!(server_hp_key[..], hex!("440b2725e91dc79b370711ef792faa3d"));
    }

    #[test]
    fn test_run_server() {
        let server_arc = run_server();

        let cloned_server_arc = server_arc.clone();
        let server_address = cloned_server_arc.local_addr().unwrap();

        let client = UdpSocket::bind("0.0.0.0:0").unwrap();
        let client_arc = Arc::new(client);

        let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let thread_tx = tx.clone();
        let thread_client = client_arc.clone();
        // let mut recv_buf: Vec<u8> = Vec::new();
        thread::spawn(move || {
            let mut recv_buf: [u8; 1500] = [0;1500];
            loop {
                let size = thread_client.recv(&mut recv_buf).unwrap();
                thread_tx.send(recv_buf[0..size].to_vec());
            }
        });
        println!("server_address: {}", server_address);
        client_arc.send_to(b"ping", server_address).unwrap();
        let ans = rx.recv().unwrap();
        println!("{:?}", std::str::from_utf8(&ans));
        let ans = rx.recv().unwrap();
        println!("{:?}", std::str::from_utf8(&ans));
        assert_eq!(1,1);
    }
}
