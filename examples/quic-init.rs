use hex_literal::hex;
use hkdf::Hkdf;
use sha2::Sha256;

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
    use crate::get_initial_keys;
    use hex_literal::hex;

    #[test]
    fn works() {
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
}

// fn main() {
//     let initial_random = hex!("0001020304050607");
//     let initial_salt = hex!("38762cf7f55934b34d179ae6a4c80cadccbb7f0a");
//
//     let (initial_secret, hk) = Hkdf::<Sha256>::extract(Some(&initial_salt), &initial_random);
//
//     assert_eq!(
//         initial_secret[..],
//         hex!("f016bb2dc9976dea2726c4e61e738a1e3680a2487591dc76b2aee2ed759822f6")
//     );
//
//     let label= expand_label("client in", 32);
//
//     assert_eq!(
//         label,
//         hex!("00200f746c73313320636c69656e7420696e00")
//     );
//
//     let mut client_secret = [0u8; 32];
//     hk.expand(label.as_slice(), &mut client_secret).unwrap();
//
//     assert_eq!(
//         client_secret[..],
//         hex!("47c6a638d4968595cc20b7c8bc5fbfbfd02d7c17cc67fa548c043ecb547b0eaa")
//     );
//
//     let hk = Hkdf::<Sha256>::from_prk(&client_secret).unwrap();
//     let mut client_key = [0u8; 16];
//     hk.expand(expand_label("quic key", 16).as_slice(), &mut client_key).unwrap();
//
//     assert_eq!(
//         client_key[..],
//         hex!("b14b918124fda5c8d79847602fa3520b"),
//     );
//
//     let hk = Hkdf::<Sha256>::from_prk(&client_secret).unwrap();
//     let mut client_iv= [0u8; 12];
//     hk.expand(expand_label("quic iv", 12).as_slice(), &mut client_iv).unwrap();
//     assert_eq!(
//         client_iv[..],
//         hex!("ddbc15dea80925a55686a7df"),
//     );
//
//     let mut client_hp_key= [0u8; 16];
//     hk.expand(expand_label("quic hp", 16).as_slice(), &mut client_hp_key).unwrap();
//     assert_eq!(
//         client_hp_key[..],
//         hex!("6df4e9d737cdf714711d7c617ee82981"),
//     );
//
// }
