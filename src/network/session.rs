use bevy::prelude::*;
use std::collections::HashMap;
use std::net::TcpStream;

pub struct Session {
    pub player_id: u32,
    pub username: String,
    pub stream: TcpStream,
}

#[derive(Resource, Default)]
pub struct SessionManager {
    pub sessions: HashMap<u32, Session>,
    pub next_id: u32,
}

impl SessionManager {
    pub fn add(&mut self, username: String, stream: TcpStream) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.sessions.insert(
            id,
            Session {
                player_id: id,
                username,
                stream,
            },
        );
        id
    }
}
