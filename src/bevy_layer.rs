use bevy::prelude::*;
use std::thread;

use crate::network;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_network_threads);
    }
}

fn start_network_threads() {
    // TCP server thread
    thread::spawn(|| {
        network::tcp::run_tcp_server();
    });

    // UDP server thread
    thread::spawn(|| {
        network::udp::run_udp_server();
    });

    println!("Networking started inside Bevy");
}
