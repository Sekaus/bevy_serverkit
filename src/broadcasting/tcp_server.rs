use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::broadcasting::packets::{TcpPacket, TcpResponse};

fn read_packet(stream: &mut TcpStream) -> TcpPacket {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();

    let len = u32::from_be_bytes(len_buf);

    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data).unwrap();

    bincode::deserialize(&data).unwrap()
}

fn write_packet<T: serde::Serialize>(stream: &mut TcpStream, packet: &T) {
    let data = bincode::serialize(packet).unwrap();
    let len = data.len() as u32;

    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(&data).unwrap();
}

fn handle_client(mut stream: TcpStream) {
    let packet = read_packet(&mut stream);

    match packet {
        TcpPacket::Login {
            username,
            invitation_key,
        } => {
            println!("Login: {username}, key: {invitation_key}");

            let response = TcpResponse::LoginSuccess {
                player_id: 1,
                token: "secret_token_123".to_string(),
                udp_port: 3001,
            };

            write_packet(&mut stream, &response);
        }
    }
}

pub fn tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("TCP running on 3000");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| handle_client(stream));
        }
    }
}
