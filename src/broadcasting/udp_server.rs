use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

use crate::broadcasting::packets::UdpPacket;

pub fn udp_server() {
    let socket = UdpSocket::bind("0.0.0.0:3001").unwrap();

    println!("UDP running on 3001");

    let mut clients: HashMap<u32, SocketAddr> = HashMap::new();
    let mut buf = [0u8; 2048];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).unwrap();

        let packet: UdpPacket = bincode::deserialize(&buf[..len]).unwrap();

        match packet {
            UdpPacket::Hello { player_id, token } => {
                println!("UDP handshake from {player_id}");

                // register player
                clients.insert(player_id, addr);
            }

            UdpPacket::Move { x, y, z } => {
                println!("Move: {x}, {y}, {z}");

                // broadcast movement to all clients
                for (_, client_addr) in &clients {
                    socket.send_to(&buf[..len], client_addr).unwrap();
                }
            }

            UdpPacket::Message { username, text } => {
                println!("[{username}] {text}");
            }

            UdpPacket::Task { action, at, value } => {
                println!("Task: {action} at {at} value {value}");
            }
        }
    }
}
