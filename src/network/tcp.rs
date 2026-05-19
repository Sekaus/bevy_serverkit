use bevy::prelude::*;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, PoisonError},
    thread,
};

use crate::network::packets::*;

#[derive(PartialEq)]
pub enum ClientState {
    Active,
    Disconnect,
}

pub struct Client {
    pub username: Option<String>,
    state: ClientState,
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            username: None,
            state: ClientState::Active,
            stream,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == ClientState::Active
    }

    /// Handles internal state update on write failure
    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        if let Err(err) = self.stream.write_all(data) {
            self.state = ClientState::Disconnect;
            return Err(err);
        }
        Ok(())
    }

    /// Centralized packet reading logic on the Client
    pub fn read_packet(&mut self) -> std::io::Result<TcpPacket> {
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf);
        let mut data = vec![0u8; len as usize];
        self.stream.read_exact(&mut data)?;

        let packet = bincode::deserialize(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(packet)
    }

    /// Helper for structured packet writing
    pub fn write_packet(&mut self, packet: &TcpResponse) -> std::io::Result<()> {
        let data = bincode::serialize(packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let len = (data.len() as u32).to_be_bytes();

        self.write(&len)?;
        self.write(&data)
    }

    pub fn try_clone_stream(&self) -> std::io::Result<TcpStream> {
        self.stream.try_clone()
    }
}

pub struct ClientList {
    connected_clients: Vec<Client>,
}

impl ClientList {
    pub fn new() -> Self {
        Self {
            connected_clients: Vec::new(),
        }
    }

    pub fn send_to(&mut self, target_username: &str, msg: &TcpResponse) {
        if let Some(client) = self
            .connected_clients
            .iter_mut()
            .find(|c| c.username.as_deref() == Some(target_username))
        {
            let _ = client.write_packet(msg);
        }
        self.connected_clients.retain(|c| c.is_active());
    }

    pub fn broadcast(&mut self, msg: &TcpResponse) {
        // Iterate and attempt writes
        for client in self.connected_clients.iter_mut() {
            // If it fails, the client.state becomes Disconnect internally
            let _ = client.write_packet(msg);
        }

        // Now explicitly clean up based on the state mutated during the loop
        self.connected_clients.retain(|c| c.is_active());
    }
}

#[derive(Resource, Clone)]
pub struct ConnectedClients {
    pub client_list: Arc<Mutex<ClientList>>,
}

impl ConnectedClients {
    pub fn new() -> Self {
        Self {
            client_list: Arc::new(Mutex::new(ClientList::new())),
        }
    }

    /// Propagates PoisonError instead of ignoring it
    pub fn connect(
        &self,
        client: Client,
    ) -> Result<(), PoisonError<std::sync::MutexGuard<ClientList>>> {
        let mut list = self.client_list.lock()?;
        list.connected_clients.push(client);
        Ok(())
    }

    pub fn broadcast(
        &self,
        msg: &TcpResponse,
    ) -> Result<(), PoisonError<std::sync::MutexGuard<ClientList>>> {
        let mut list = self.client_list.lock()?;
        list.broadcast(msg);
        Ok(())
    }
}

fn handle_connected_client(mut client: Client, clients_controller: ConnectedClients) {
    loop {
        match client.read_packet() {
            Ok(packet) => match packet {
                TcpPacket::Login { username } => {
                    client.username = Some(username.clone());
                    println!("[{username}] logged in");
                }
                TcpPacket::Chat { username, message } => {
                    println!("[{username}] {message}");
                    if let Err(e) =
                        clients_controller.broadcast(&TcpResponse::Chat { username, message })
                    {
                        eprintln!("Critical Lock Poisoning: {}", e);
                        break;
                    }
                }
                // Add this missing arm:
                TcpPacket::PrivateChat { target, message } => {
                    println!("Private message request for {}, with {}", target, message);
                    // Logic to find 'target' in your client list and send would go here
                }
            },
            Err(_) => break,
        }
    }
}

pub fn run_tcp_server_with_clients(connected_clients: ConnectedClients) {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("Bind failed: Port 3000 busy");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let thread_stream = stream.try_clone().expect("Stream clone failed");
            let client_for_list = Client::new(stream);
            let client_for_thread = Client::new(thread_stream);

            connected_clients
                .connect(client_for_list)
                .expect("Lock poisoned");

            let clients_ref = connected_clients.clone();
            thread::spawn(move || {
                handle_connected_client(client_for_thread, clients_ref);
            });
        }
    }
}
