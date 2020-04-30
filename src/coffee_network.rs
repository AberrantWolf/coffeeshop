pub mod ui;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};

use tokio::prelude::*;

pub struct PeerInfo {
    stream: TcpStream,
    address: SocketAddr,
}

struct PeerContainer<T> {
    tx: WriteHalf<T>,
    address: SocketAddr,
}

enum Message {
    Ping,
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    TextChat(SocketAddr, String),
    VoiceChat(SocketAddr, Vec<u8>),
}

pub struct NetworkStateInner {
    address: SocketAddr,
    peers: HashMap<SocketAddr, PeerContainer<TcpStream>>,
    // Broadcase for sending messages OUT from the network state
    broadcast_tx: broadcast::Sender<Message>,
    broadcast_rx: broadcast::Receiver<Message>,
    // MPSC for sending messages INTO the network state
    mpsc_tx: mpsc::Sender<Message>,
    // mpsc_rx: mpsc::Receiver<Message>,
}

impl NetworkStateInner {
    fn add_peer(&mut self, info: PeerContainer<TcpStream>) {
        self.peers.insert(info.address, info);
    }

    fn get_address(&self) -> SocketAddr {
        self.address.clone()
    }

    fn get_to_server_tx(&self) -> mpsc::Sender<Message> {
        self.mpsc_tx.clone()
    }

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Ping => unimplemented!(),
            Message::Connect(addr) => unimplemented!(),
            Message::Disconnect(addr) => unimplemented!(),
            Message::TextChat(addr, text) => unimplemented!(),
            Message::VoiceChat(addr, byte_vec) => unimplemented!(),
        }
    }
}

impl PeerInfo {
    fn new(stream: TcpStream, address: SocketAddr) -> Self {
        PeerInfo { stream, address }
    }

    fn get_stream(self) -> TcpStream {
        self.stream
    }

    fn get_address(&self) -> SocketAddr {
        self.address
    }
}

type NetworkState = Arc<Mutex<NetworkStateInner>>;
// TODO: this doesn't need to be separate from the `start_server` function
pub fn create_network() -> NetworkState {
    let (btx, brx) = broadcast::channel::<Message>(16);
    let (mtx, mut mrx) = mpsc::channel::<Message>(100);

    let state = Arc::new(Mutex::new(NetworkStateInner {
        address: SocketAddr::from(([127, 0, 0, 1], 0)),
        peers: HashMap::new(),
        broadcast_tx: btx, // clonable, can send across threads
        broadcast_rx: brx, // not clonable -- make new from a sender
        mpsc_tx: mtx,      // clonable
    }));

    // Initiate async loop to receive
    let result_state = state.clone();
    tokio::spawn(async move {
        loop {
            if let Some(msg) = mrx.recv().await {
                state.lock().unwrap().handle_message(msg);
            }
        }
    });

    result_state
}

pub fn start_server(state: NetworkState, mut cb: Box<dyn FnMut(&PeerInfo) + Send>) {
    // Spawn process to listen for new network connections
    tokio::spawn(async move {
        let address = state.lock().unwrap().get_address();
        let mut listener = TcpListener::bind(address).await.unwrap();
        loop {
            let (stream, address) = listener.accept().await.unwrap();
            let peer_info = PeerInfo { stream, address };
            cb(&peer_info);

            let state = state.clone();
            process_new_peer(state, peer_info);
        }
    });
}

fn process_new_peer(state: NetworkState, peer: PeerInfo) {
    let address = peer.get_address();
    let stream = peer.get_stream();
    let (mut rx, tx) = tokio::io::split(stream);

    // TODO: I think I can remove the peer tracking unless I want to
    // be able to query specifically how many peers there are...?
    let peer = PeerContainer { tx, address };
    state.lock().unwrap().add_peer(peer);

    let mut server_tx = state.lock().unwrap().get_to_server_tx();
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let read = rx.read(&mut buf).await;
            let count = match read {
                Ok(c) => c,
                Err(_) => {
                    break;
                }
            };
            if count > 0 {
                // TODO: do something with the contents of buf...
                let msg = Message::TextChat(
                    address,
                    std::str::from_utf8(&buf[..count]).unwrap().to_owned(),
                );
                if let Err(_err) = server_tx.send(msg).await {
                    break;
                }
            }
        }
        // TODO: ...remove from the network state...?
        // (requires clone of address, though)
    });

    tokio::spawn(async move {
        // TODO: subscribe to the state's broadcast channel and send
        // messages back out -- provided that they didn't come from us
    });
}
