pub mod ui;

mod peer;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::{broadcast, mpsc, Mutex};

use uuid::Uuid;

use self::peer::{Peer, PeerInfo};

#[derive(Clone, Debug)]
pub enum Message {
    _Connect(Uuid),
    Disconnect(Uuid),
    TextChat(Uuid, String),
    _VoiceChat(Uuid, Vec<u8>),
}

#[derive(Debug)]
struct InnerNetworkState {
    address: SocketAddr,
    peers: Vec<Peer>,
}

#[derive(Clone, Debug)]
pub struct NetworkState {
    info: PeerInfo,
    // Broadcase for sending messages OUT from the network state
    broadcast_tx: broadcast::Sender<Message>,
    // MPSC for sending messages INTO the network state
    mpsc_tx: mpsc::Sender<Message>,
    inner: Arc<Mutex<InnerNetworkState>>,
}

impl NetworkState {
    pub fn new() -> Self {
        const PORT_NUM: u16 = 0;
        let (btx, _brx) = broadcast::channel::<Message>(16);
        let (mtx, mrx) = mpsc::channel::<Message>(100);
        let state = NetworkState {
            info: PeerInfo::new(Uuid::new_v4(), "Test".to_string()),
            broadcast_tx: btx,
            mpsc_tx: mtx,
            inner: Arc::new(Mutex::new(InnerNetworkState {
                address: SocketAddr::from(([0, 0, 0, 0], PORT_NUM)),
                peers: vec![],
            })),
        };

        // Initiate async loop to receive on MPSC channel
        state.start_mpsc(mrx);

        // Initiate async loop to accept remote connections
        state.start_tcp();

        state
    }

    async fn add_peer(&mut self, peer: Peer) {
        self.inner.lock().await.peers.push(peer)
    }

    async fn handle_message(&mut self, msg: Message) {
        println!("Handling message: {:?}", msg);
        // TODO: do we need to do any processing of the message?

        // Rebroadcast all messages (for now) to all listeners
        if self.broadcast_tx.send(msg).is_err() {
            // TODO: report error to a proper logger
        }
    }

    fn start_mpsc(&self, mut mrx: mpsc::Receiver<Message>) {
        let mut state = self.clone();
        tokio::spawn(async move {
            loop {
                if let Some(msg) = mrx.recv().await {
                    state.handle_message(msg).await;
                }
            }
        });
    }

    fn start_tcp(&self) {
        let mut state = self.clone();
        tokio::spawn(async move {
            let address = state.get_address().await;
            let mut listener = TcpListener::bind(address).await.unwrap();
            if let Ok(address) = listener.local_addr() {
                state.set_address(address);
                println!("Listener bound: {:?}", address);
                loop {
                    let (mut stream, address) = listener.accept().await.unwrap();
                    let state = state.clone();

                    let bytes = bincode::serialize(&state.info).unwrap();
                    if stream.write(&bytes).await.is_ok() {
                        process_new_peer(state, address, stream);
                    }
                }
            } else {
                println!("Error binding network listener");
            }
        });
    }

    pub fn connect_to(&self, address: String) {
        println!("Connecting to: {}", address);
        let state = self.clone();
        tokio::spawn(async move {
            if let Ok(address) = address.parse() {
                match TcpStream::connect(address).await {
                    Ok(mut stream) => {
                        if let Ok(v) = bincode::serialize::<PeerInfo>(&state.info) {
                            if stream.write(&v).await.is_ok() {
                                process_new_peer(state, address, stream);
                            } else {
                                println!("Error writing handshake data on tcp stream");
                            }
                        } else {
                            println!("Error serializing handshake info");
                        }
                    }
                    Err(e) => {
                        // TODO: report an error to a proper logger
                        println!("Server connection failed: {}, {}", address, e);
                    }
                }
            } else {
                println!("Unable to parse address");
            }
        });
    }

    pub async fn get_address(&self) -> SocketAddr {
        self.inner.lock().await.address
    }
    fn set_address(&mut self, address: SocketAddr) {
        let net = self.clone();
        tokio::spawn(async move { net.inner.lock().await.address = address });
    }

    pub fn get_server_sender(&self) -> mpsc::Sender<Message> {
        self.mpsc_tx.clone()
    }

    pub fn get_broadcast_receiver(&self) -> broadcast::Receiver<Message> {
        self.broadcast_tx.subscribe()
    }

    pub fn send_text_message(&self, text: String) {
        let net = self.clone();
        tokio::spawn(async move {
            let mut sender = net.get_server_sender();
            if let Err(e) = sender
                .send(Message::TextChat(net.info.id(), text.clone()))
                .await
            {
                println!("Error sending text message: {}", e);
            }
            println!("Sent message: {}", text);
        });
    }
}

fn process_new_peer(mut state: NetworkState, addr: SocketAddr, stream: TcpStream) {
    println!("Accepting peer: {}", addr);

    tokio::spawn(async move {
        let peer = match Peer::new_from_stream(stream).await {
            Ok(peer) => peer,
            Err(_e) => {
                // TODO: Log e somewhere
                return;
            }
        };
        state.add_peer(peer.clone()).await;
        peer.run(state.clone()).await;
    });
}
