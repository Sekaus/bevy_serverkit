#[path = "../src/broadcasting/mod.rs"]
mod broadcasting;
use crate::broadcasting::tcp_client::tcp_client;
use crate::broadcasting::tcp_server::tcp_server;
use crate::broadcasting::udp_client::udp_client;
use crate::broadcasting::udp_server::udp_server;

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        tcp_server();
    });

    thread::spawn(|| {
        udp_server();
    });

    // give servers time to start
    thread::sleep(Duration::from_millis(500));

    thread::spawn(|| {
        tcp_client("Player1".to_string(), "inv_key_123".to_string());
    });

    thread::spawn(|| {
        udp_client();
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
