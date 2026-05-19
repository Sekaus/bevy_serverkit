use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::network::packets::*;

type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn read_packet(stream: &mut TcpStream) -> TcpPacket {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();

    let len = u32::from_be_bytes(len_buf);
    let mut data = vec![0u8; len as usize];

    stream.read_exact(&mut data).unwrap();

    bincode::deserialize(&data).unwrap()
}

fn write_packet(stream: &mut TcpStream, packet: &TcpResponse) {
    let data = bincode::serialize(packet).unwrap();
    let len = data.len() as u32;

    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(&data).unwrap();
}

fn broadcast(clients: &Clients, msg: &TcpResponse) {
    let data = bincode::serialize(msg).unwrap();
    let len = data.len() as u32;

    let mut clients = clients.lock().unwrap();

    clients.retain_mut(|c| {
        c.write_all(&len.to_be_bytes())
            .and_then(|_| c.write_all(&data))
            .is_ok()
    });
}

fn handle_client(mut stream: TcpStream, clients: Clients) {
    loop {
        let packet = read_packet(&mut stream);

        match packet {
            TcpPacket::Login { username } => {
                println!("Login: {username}");

                clients.lock().unwrap().push(stream.try_clone().unwrap());
            }

            TcpPacket::Chat { username, message } => {
                println!("[{username}] {message}");

                broadcast(&clients, &TcpResponse::Chat { username, message });
            }
        }
    }
}

pub fn run_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("TCP chat server running on 3000");

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let clients = clients.clone();

            thread::spawn(move || {
                handle_client(stream, clients);
            });
        }
    }
}
