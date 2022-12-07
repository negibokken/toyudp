use super::address::Address;

#[derive(Debug, Clone)]
pub struct Datagram {
    pub bytes: Vec<u8>,
    dest: Option<Address>,
    src: Option<Address>,
}

impl Datagram {
    pub fn new(bytes: Vec<u8>, dest: Option<Address>, src: Option<Address>) -> Datagram {
        Datagram { bytes, dest, src }
    }

    pub fn dest(&self) -> Option<Address> {
        self.dest
    }

    pub fn src(&self) -> Option<Address> {
        self.src
    }
}
