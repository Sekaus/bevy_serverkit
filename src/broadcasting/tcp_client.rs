use std::io::{Read, Write};
use std::net::TcpStream;

use crate::broadcasting::packets::{TcpPacket, TcpResponse};

fn write_packet<T: serde::Serialize>(stream: &mut TcpStream, packet: &T) {
    let data = bincode::serialize(packet).unwrap();
    let len = data.len() as u32;

    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(&data).unwrap();
}

fn read_packet(stream: &mut TcpStream) -> TcpResponse {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();

    let len = u32::from_be_bytes(len_buf);

    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data).unwrap();

    bincode::deserialize(&data).unwrap()
}

pub fn tcp_client(name: String, key: String) {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let login = TcpPacket::Login {
        username: name,
        invitation_key: key,
    };

    write_packet(&mut stream, &login);

    let response = read_packet(&mut stream);

    println!("TCP response: {:?}", response);
}
