use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::coffee_network::{Message, NetworkController};

#[derive(Serialize, Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PeerInfo {
    id: Uuid,
    nickname: String,
    udp_port: u16,
}

impl PeerInfo {
    pub fn new(id: Uuid, nickname: String, udp_port: u16) -> Self {
        PeerInfo {
            id,
            nickname,
            udp_port,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn set_udp_port(&mut self, port: u16) {
        self.udp_port = port
    }
}

#[derive(Clone, Debug)]
pub struct Peer {
    info: PeerInfo,
    tcp_stream: Arc<RwLock<TcpStream>>,
    udp_port: u16,
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
            tcp_stream: Arc::new(RwLock::new(tcp_stream)),
            // TODO: UDP port goes here...
        })
    }

    async fn tcp_read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        // println!("trying to read");
        self.tcp_stream.write().await.read(bytes).await
    }

    async fn tcp_write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tcp_stream.write().await.write(bytes).await
    }

    pub async fn run(&self, net: NetworkController) {
        let mut server_tx = net.get_server_sender().await;
        let mut broadcast_rx = net.get_broadcast_receiver().await;

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
                        if let Ok(peer_message) = bincode::deserialize::<PeerMessageTcp>(&buf[..count]) {
                            match peer_message {
                                PeerMessageTcp::Ping => {}, // TODO
                                PeerMessageTcp::Pong => {}, // TODO
                                PeerMessageTcp::ChatEvent(sender, text) => {
                                    if sender == net.get_local_peer_info().await.id {
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
                                    let peer_message = PeerMessageTcp::ChatEvent(sender, text);
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
enum PeerMessageTcp {
    Ping,
    Pong,
    ChatEvent(Uuid, String),
}
