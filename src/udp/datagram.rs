use super::address::Address;

#[derive(Debug,Clone)]
pub struct Datagram {
    pub bytes: Vec<u8>,
    pub dest: Option<Address>,
    pub src: Option<Address>
}

impl Datagram {
    pub fn new (bytes: Vec<u8>, dest: Option<Address>, src: Option<Address> ) -> Datagram {
        Datagram {
            bytes,
            dest,
            src,
        }
    }
}
