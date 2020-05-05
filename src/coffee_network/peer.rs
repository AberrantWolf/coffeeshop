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

        let info = bincode::deserialize::<PeerInfo>(&buf[..read_count])?;

        Ok(Peer {
            info,
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
        })
    }

    async fn tcp_read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tcp_stream.lock().await.read(bytes).await
    }

    async fn tcp_write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tcp_stream.lock().await.write(bytes).await
    }

    pub async fn run(&self, state: NetworkState) {
        let mut server_tx = state.get_server_sender();
        let mut broadcast_rx = state.get_broadcast_receiver();

        // TODO: create a UDP connection with this peer for sending/receiving audio data

        let mut peer = self.clone();
        let mut buf = [0u8; 1024];
        loop {
            tokio::select! {
                read = peer.tcp_read(&mut buf) => {
                    let count = match read {
                        Ok(c) => c,
                        Err(_) => {
                            break;
                        }
                    };
                    if count > 0 {
                        // TODO: maybe need to wait for a null terminator and
                        // break messages apart?
                        let text = std::str::from_utf8(&buf[..count]).unwrap().to_owned();
                        println!("Message received: {}", text);
                        let msg = Message::TextChat(peer.info.id, text);
                        if let Err(_err) = server_tx.send(msg).await {
                            break;
                        }
                    }
                },
                recv_result = broadcast_rx.recv() => {
                    match recv_result {
                        Ok(msg) => {
                            match msg {
                                Message::_Connect(_) => {}
                                Message::Disconnect(_) => {}
                                // Message::Ping => {}
                                Message::TextChat(sender, text) => {
                                    if sender != peer.info.id && peer.tcp_write(text.as_bytes()).await.is_err() {
                                        break;
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

        // Send the server a message that we are disconnecting
        if let Err(_err) = server_tx.send(Message::Disconnect(peer.info.id)).await {
            return;
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
enum PeerRequest {
    Ping,
    Pong,
    // ChatRequest()
}
