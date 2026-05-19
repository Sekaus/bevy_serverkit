use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TcpPacket {
    Login {
        username: String,
        invitation_key: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TcpResponse {
    LoginSuccess {
        player_id: u32,
        token: String,
        udp_port: u16,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UdpPacket {
    Hello {
        player_id: u32,
        token: String,
    },

    Message {
        username: String,
        text: String,
    },

    Move {
        x: f32,
        y: f32,
        z: f32,
    },

    Task {
        action: String,
        at: String,
        value: String,
    },
}
