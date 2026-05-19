use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TcpPacket {
    Login { username: String },
    Chat { username: String, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TcpResponse {
    Chat { username: String, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UdpPacket {
    Ping,
}
