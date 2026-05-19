use std::net::UdpSocket;

use crate::broadcasting::packets::UdpPacket;

pub fn udp_client() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.connect("127.0.0.1:3001").unwrap();

    // handshake
    let hello = UdpPacket::Hello {
        player_id: 1,
        token: "secret_token_123".to_string(),
    };

    socket.send(&bincode::serialize(&hello).unwrap()).unwrap();

    // send movement
    let mv = UdpPacket::Move {
        x: 10.0,
        y: 5.0,
        z: -2.0,
    };

    socket.send(&bincode::serialize(&mv).unwrap()).unwrap();

    let mut buf = [0u8; 2048];

    loop {
        let len = socket.recv(&mut buf).unwrap();

        let packet: UdpPacket = bincode::deserialize(&buf[..len]).unwrap();

        println!("UDP recv: {:?}", packet);
    }
}
