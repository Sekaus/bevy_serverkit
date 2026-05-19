use bevy::prelude::*;

use std::thread;

use crate::network;

use crate::network::tcp::ConnectedClients;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        // 1. Create the shared state first
        let connected_clients = ConnectedClients::new();

        // 2. Register it as a resource so Bevy systems can use it
        app.insert_resource(connected_clients.clone());

        // 3. Move the shared state into the startup system
        app.add_systems(Startup, move || {
            let clients_for_thread = connected_clients.clone();

            // Spawn TCP server and pass the shared resource
            thread::spawn(move || {
                network::tcp::run_tcp_server_with_clients(clients_for_thread);
            });

            // Spawn UDP server
            thread::spawn(|| {
                network::udp::run_udp_server();
            });

            println!("Networking started inside Bevy");
        });
    }
}
