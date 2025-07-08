use std::net::TcpListener;
use std::slice::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Header {
    question: [u8; 256],
    id: u16,
    qr: u16,
    opcode: u8,
    AA: bool,
    RCODE: u8,
}

#[derive(Serialize, Deserialize)]
struct Question {
    qname: Vec<u8>, // arbitrary sequence of labels encoded as bytes
    qclass: u16,
    qtype: u16,
}


#[derive(Serialize, Deserialize)]
struct DNSMessage {
    header: Header, // make these into slices if size is known
    questions: Question,
    answer: ,
    additional: Vec<u8>,
    authority: Vec<u8>
}


fn main() {
    let mut test_packet: Vec<u8> = vec![];

}
