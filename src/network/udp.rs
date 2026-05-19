use std::net::UdpSocket;

use crate::network::packets::UdpPacket;

pub fn run_udp_server() {
    let socket = UdpSocket::bind("0.0.0.0:3001").unwrap();
    println!("UDP heartbeat server on 3001");

    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).unwrap();

        let packet: UdpPacket = bincode::deserialize(&buf[..len]).unwrap();

        if let UdpPacket::Ping = packet {
            let _ = socket.send_to(b"pong", addr);
        }
    }
}
