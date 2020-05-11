use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpStream, UdpSocket};
use tokio::prelude::*;
use tokio::sync::{broadcast, mpsc, RwLock};
use uuid::Uuid;

use crate::coffee_network::{Message, NetworkController};

#[derive(Serialize, Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
struct PeerInfo {
    id: Uuid,
    nickname: String,
    udp_port: u16,
}

#[derive(Debug)]
struct PeerPrivate {
    tcp_stream: TcpStream,
    udp_socket: UdpSocket,
    udp_pong_ok: bool,
    broadcast_rx: broadcast::Receiver<Message>,
    server_tx: mpsc::Sender<Message>,
}

#[derive(Clone, Debug)]
pub struct Peer {
    info: PeerInfo,
    inner: Arc<RwLock<PeerPrivate>>,
}

impl Peer {
    pub async fn new(
        mut tcp_stream: TcpStream,
        net: NetworkController,
    ) -> Result<Self, Box<dyn Error>> {
        let mut buf = [0u8; 1024];

        // Open UDP conneciton and get port number
        let mut local_address = net.get_address().await;
        local_address.set_port(0); // bind to dynamic port
        let udp_socket = UdpSocket::bind(local_address).await.unwrap();
        let udp_port = udp_socket.local_addr().unwrap().port();

        // Construct local peer info to send to remote
        let local_peer_info = PeerInfo {
            id: net.get_local_id().await,
            nickname: net.get_local_nick().await,
            udp_port,
        };

        // Write to remote
        if let Ok(v) = bincode::serialize::<PeerInfo>(&local_peer_info) {
            if tcp_stream.write(&v).await.is_ok() {
                println!("Sending local peer info: {:?}", local_peer_info);
            } else {
                println!("Error writing handshake data on tcp stream");
            }
        } else {
            println!("Error serializing handshake info");
        }

        // Receive initial PeerInfo from the remote connection
        let read_count = tcp_stream.read(&mut buf).await?;
        let info = bincode::deserialize::<PeerInfo>(&buf[..read_count])?;
        println!("Received remote peer info: {:?}", info);

        // Connect the UDP socket to remote's address and  UDP port
        let mut remote_address = tcp_stream.peer_addr()?;
        remote_address.set_port(info.udp_port);
        udp_socket.connect(remote_address).await?;

        // Create server channel bindings
        let server_tx = net.get_server_sender().await;
        let broadcast_rx = net.get_broadcast_receiver().await;
        let peer = Peer {
            info,
            inner: Arc::new(RwLock::new(PeerPrivate {
                tcp_stream,
                udp_socket,
                udp_pong_ok: false,
                broadcast_rx,
                server_tx,
            })),
        };
        peer.start_polling();

        Ok(peer)
    }

    // UDP fns
    async fn is_udp_pong_ok(&self) -> bool {
        self.inner.read().await.udp_pong_ok
    }

    async fn set_udp_pong_ok(&mut self) {
        self.inner.write().await.udp_pong_ok = true
    }

    async fn udp_read(&self, bytes: &mut [u8]) -> io::Result<usize> {
        self.inner.write().await.udp_socket.recv(bytes).await
    }

    async fn udp_write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.inner.write().await.udp_socket.send(bytes).await
    }

    async fn handle_udp_read(&mut self, read: io::Result<usize>, bytes: &[u8]) -> Result<(), ()> {
        println!("Peer received udp signal");
        if let Ok(peer_message) = PeerMessageUdp::new_from_read(read, &bytes) {
            match peer_message {
                PeerMessageUdp::Ping => {
                    println!("Received UDP PING!!");
                    if let Ok(ping_bytes) = bincode::serialize(&PeerMessageUdp::Pong {}) {
                        if self.udp_write(&ping_bytes).await.is_err() {
                            println!("Error sending  UDP pong");
                        }
                    }
                }
                PeerMessageUdp::Pong => {
                    println!("Received UDP PONG!!");
                    self.set_udp_pong_ok().await;
                }
                PeerMessageUdp::VoiceData(_, _) => {}
            }
        }
        Ok(())
    }

    // TCP fns
    async fn tcp_read(&self, bytes: &mut [u8]) -> io::Result<usize> {
        self.inner.write().await.tcp_stream.read(bytes).await
    }

    async fn tcp_write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        println!("Sending TCP to peer: {:?}", self);
        self.inner.write().await.tcp_stream.write(bytes).await
    }

    async fn handle_tcp_read(&mut self, read: io::Result<usize>, bytes: &[u8]) -> Result<(), ()> {
        println!("Peer received tcp signal");
        let count = match read {
            Ok(c) => c,
            Err(e) => {
                println!("Error reading TCP: {}", e);
                return Err(());
            }
        };
        if count < 1 {
            println!("No data read (count < 1)");
            return Err(());
        }

        // TODO: maybe need to wait for a null terminator and
        // break messages apart?
        if let Ok(peer_message) = bincode::deserialize::<PeerMessageTcp>(&bytes[..count]) {
            match peer_message {
                PeerMessageTcp::Ping => {} // TODO
                PeerMessageTcp::Pong => {} // TODO
                PeerMessageTcp::ChatEvent(_sender, text) => {
                    println!("Message received: {}", text);
                    let msg = Message::TextChat(self.info.id, text);
                    if let Err(_err) = self.server_send(msg).await {
                        return Err(());
                    }
                }
            }
        } else {
            println!("Error deserializing message");
        }
        Ok(())
    }

    async fn server_recv(&self) -> Result<Message, broadcast::RecvError> {
        self.inner.write().await.broadcast_rx.recv().await
    }

    async fn server_send(&mut self, msg: Message) -> Result<(), mpsc::error::SendError<Message>> {
        self.inner.write().await.server_tx.send(msg).await
    }

    // TODO: Make this return result so that loop can fail on failure
    async fn wait_for_udp_ping(&self) {
        let mut udp_buf = [0u8; 1024];
        let mut peer = self.clone();
        loop {
            let mut delay = tokio::time::delay_for(tokio::time::Duration::from_millis(500));
            tokio::select! {
                _ = &mut delay => {
                    if let Ok(ping_bytes) = bincode::serialize(&PeerMessageUdp::Ping {}) {
                        if peer.udp_write(&ping_bytes).await.is_err() {
                            println!("Error sending  UDP ping");
                            break;
                        }
                        println!("Sending UDP Ping");
                    }
                },
                udp_read = self.udp_read(&mut udp_buf) => {
                    println!("Read UDP...");
                    if peer.handle_udp_read(udp_read, &udp_buf).await.is_ok() && self.is_udp_pong_ok().await {
                        break;
                    }
                }
            };
        }
    }

    fn start_polling(&self) {
        let mut peer = self.clone();
        let mut udp_buf = [0u8; 1024];
        let mut tcp_buf = [0u8; 1024];
        tokio::spawn(async move {
            if let Ok(ping_bytes) = bincode::serialize(&PeerMessageUdp::Ping {}) {
                if peer.udp_write(&ping_bytes).await.is_err() {
                    println!("Error sending  UDP ping");
                    return;
                }
            }
            // TODO: Check result for failure here
            peer.wait_for_udp_ping().await;
            println!("Starting peer poll loop... {:?}", peer);
            loop {
                println!("Inside loop...");
                tokio::select! {
                    // udp_read = peer.udp_read(&mut udp_buf) => {
                    //     println!("UDP came in to peer");
                    //     if peer.handle_udp_read(udp_read, &udp_buf).await.is_err() {break;}
                    // },
                    tcp_read = peer.tcp_read(&mut tcp_buf) => {
                        println!("TCP came in to peer");
                        if peer.handle_tcp_read(tcp_read, &tcp_buf).await.is_err() {break;}
                    },
                    recv_result = peer.server_recv() => {
                        println!("Broadcast came in to peer");
                        match recv_result {
                            Ok(msg) => {
                                println!("Peer received broadcast: {:?}", msg);
                                match msg {
                                    Message::_Connect(_) => {}
                                    Message::Disconnect(_) => {}
                                    Message::TextChat(sender, text) => {
                                        if sender == peer.info.id {
                                            println!("Refusing to send message back to myself");
                                            continue;
                                        }
                                        let peer_message = PeerMessageTcp::ChatEvent(sender, text);
                                        if let Ok(bytes) = bincode::serialize(&peer_message) {
                                            if peer.tcp_write(&bytes).await.is_err() {
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
                };
            }
            print!("Peer disconnecting {}:", peer.info.id);

            // Send the server a message that we are disconnecting
            if let Err(_err) = peer.server_send(Message::Disconnect(peer.info.id)).await {
                return;
            }
        });
    }
}

#[derive(Deserialize, Serialize, Clone)]
enum PeerMessageTcp {
    Ping,
    Pong,
    ChatEvent(Uuid, String),
}

#[derive(Deserialize, Serialize, Clone)]
enum PeerMessageUdp {
    Ping,
    Pong,
    VoiceData(Uuid, Vec<u8>),
}

impl PeerMessageUdp {
    fn new_from_read(read: io::Result<usize>, bytes: &[u8]) -> Result<Self, ()> {
        let count = match read {
            Ok(c) => c,
            Err(e) => {
                println!("Error reading TCP: {}", e);
                return Err(());
            }
        };
        if count < 1 {
            return Err(());
        }

        if let Ok(message) = bincode::deserialize::<PeerMessageUdp>(&bytes[..count]) {
            return Ok(message);
        }

        Err(())
    }
}
