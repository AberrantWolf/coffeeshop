use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::coffee_network::{Message, NetworkState};

#[derive(Serialize, Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PeerInfo {
    id: Uuid,
    nickname: String,
}

impl PeerInfo {
    pub fn new(id: Uuid, nickname: String) -> Self {
        PeerInfo { id, nickname }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Peer {
    info: PeerInfo,
    tcp_stream: Arc<Mutex<TcpStream>>,
}

impl Peer {
    pub async fn new_from_stream(mut tcp_stream: TcpStream) -> Result<Self, Box<dyn Error>> {
        let mut buf = [0u8; 1024];
        let read_count = tcp_stream.read(&mut buf).await?;
        println!("Creating new peer from stream...");

        let info = bincode::deserialize::<PeerInfo>(&buf[..read_count])?;
        println!("New peer info: {}, {}", info.id, info.nickname);

        Ok(Peer {
            info,
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
        })
    }

    async fn tcp_read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        // println!("trying to read");
        self.tcp_stream.lock().await.read(bytes).await
    }

    async fn tcp_write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tcp_stream.lock().await.write(bytes).await
    }

    pub async fn run(&self, state: NetworkState) {
        let mut server_tx = state.get_server_sender();
        let mut broadcast_rx = state.get_broadcast_receiver();

        // TODO: create a UDP connection with this peer for sending/receiving audio data
        // TODO: make separate mpsc/broadcast channels for text/voice comms and subscribe based on client desired capabilities

        let mut peer = self.clone();
        let mut buf = [0u8; 1024];
        loop {
            tokio::select! {
                read = peer.tcp_read(&mut buf) => {
                    println!("Peer received tcp signal");
                    let count = match read {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Error reading TCP: {}", e);
                            break;
                        }
                    };
                    if count > 0 {
                        // TODO: maybe need to wait for a null terminator and
                        // break messages apart?
                        if let Ok(peer_message) = bincode::deserialize::<PeerMessage>(&buf[..count]) {
                            match peer_message {
                                PeerMessage::Ping => {}, // TODO
                                PeerMessage::Pong => {}, // TODO
                                PeerMessage::ChatEvent(sender, text) => {
                                    if sender == state.info.id {
                                        continue;
                                    }
                                    println!("Message received: {}", text);
                                    let msg = Message::TextChat(peer.info.id, text);
                                    if let Err(_err) = server_tx.send(msg).await {
                                        break;
                                    }
                                },
                            }
                        } else {
                            println!("Error deserializing message");
                        }
                    } else {
                        println!("No data read (count < 1)");
                    }
                },
                recv_result = broadcast_rx.recv() => {
                    match recv_result {
                        Ok(msg) => {
                            println!("Peer received broadcast: {:?}", msg);
                            match msg {
                                Message::_Connect(_) => {}
                                Message::Disconnect(_) => {}
                                Message::TextChat(sender, text) => {
                                    let peer_message = PeerMessage::ChatEvent(sender, text);
                                    if let Ok(bytes) = bincode::serialize(&peer_message) {
                                        if sender != peer.info.id && peer.tcp_write(&bytes).await.is_err() {
                                            println!("Error sending text chat");
                                            break;
                                        }
                                    } else {
                                        println!("Error  converting message to bytes");
                                    }
                                }
                                Message::_VoiceChat(sender, _bytes) => {
                                    if sender != peer.info.id {
                                        // TODO: encode and send the data over UDP
                                    }
                                }
                            }
                        },
                        Err(_e) => {},
                    }
                },
            }
        }
        print!("Peer disconnecting {}:", peer.info.id);

        // Send the server a message that we are disconnecting
        if let Err(_err) = server_tx.send(Message::Disconnect(peer.info.id)).await {
            return;
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
enum PeerMessage {
    Ping,
    Pong,
    ChatEvent(Uuid, String),
}
