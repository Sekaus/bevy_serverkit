#[path = "../src/bevy_layer.rs"]
mod bevy_layer;
#[path = "../src/network/mod.rs"]
mod network;

use bevy::prelude::*;

use network::{
    packets::{TcpPacket, TcpResponse},
    tcp::ConnectedClients,
};

use std::{
    io::{Read, Write},
    thread,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_layer::NetworkingPlugin)
        .add_systems(Startup, setup_mock_client)
        .add_systems(Update, send_message_on_input)
        .run();
}

/// System that sends a message to a specific user ("Alice") when 'M' is pressed
fn send_message_on_input(keyboard: Res<ButtonInput<KeyCode>>, network: Res<ConnectedClients>) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        let response = TcpResponse::PrivateMessage {
            sender: "Server".to_string(),
            message: "Hello Alice! This is a targeted message.".to_string(),
        };

        // Access the shared network state and send to Alice
        if let Ok(mut list) = network.client_list.lock() {
            list.send_to("Alice", &response);
            println!("Attempted to send message to Alice");
        }
    }
}

/// System that sends a message to a specific user ("Alice") when 'M' is pressed
fn setup_mock_client() {
    thread::spawn(move || {
        if let Ok(mut stream) = std::net::TcpStream::connect("127.0.0.1:3000") {
            // 1. Send Login
            let login = TcpPacket::Login {
                username: "Alice".to_string(),
            };
            send_packet(&mut stream, &login);
            println!("Mock Alice connected and logged in.");

            // 2. Continuous Read Loop to see the private message
            let mut len_buf = [0u8; 4];
            while let Ok(_) = stream.read_exact(&mut len_buf) {
                let len = u32::from_be_bytes(len_buf);
                let mut data = vec![0u8; len as usize];
                stream.read_exact(&mut data).unwrap();

                let response: TcpResponse = bincode::deserialize(&data).unwrap();
                match response {
                    TcpResponse::PrivateMessage { sender, message } => {
                        println!(">>> [CLIENT ALICE RECEIVED] from {}: {}", sender, message);
                    }
                    _ => println!("Received other response: {:?}", response),
                }
            }
        }
    });
}

// Helper to handle the length-prefix protocol
fn send_packet(stream: &mut std::net::TcpStream, packet: &TcpPacket) {
    let data = bincode::serialize(packet).unwrap();
    let len = (data.len() as u32).to_be_bytes();
    let _ = stream.write_all(&len);
    let _ = stream.write_all(&data);
}
