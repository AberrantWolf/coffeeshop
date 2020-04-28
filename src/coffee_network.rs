pub mod ui;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

use tokio::prelude::*;

pub struct PeerInfo {
    stream: TcpStream,
    address: SocketAddr,
}

struct PeerContainer<T> {
    tx: WriteHalf<T>,
    address: SocketAddr,
}

#[derive(Default)]
pub struct NetworkStateInner {
    address: Option<SocketAddr>,
    peers: Vec<PeerContainer<TcpStream>>,
}

impl NetworkStateInner {
    fn add_peer(&mut self, info: PeerContainer<TcpStream>) {
        self.peers.push(info);
    }

    fn set_address(&mut self, address: SocketAddr) {
        self.address = Some(address);
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

pub type NetworkState = Arc<Mutex<NetworkStateInner>>;

pub fn start_server(state: NetworkState, mut cb: Box<dyn FnMut(&PeerInfo) + Send>) {
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], 0));
    // let app_state = app_state.clone();
    tokio::spawn(async move {
        let mut listener = TcpListener::bind(&listen_addr).await.unwrap();
        {
            state
                .clone()
                .lock()
                .unwrap()
                .set_address(listener.local_addr().unwrap());
        }
        // app_state
        //     .lock()
        //     .unwrap()
        //     .port_text
        //     .set_content(format!("{}", listener.local_addr().unwrap()));
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

    let peer = PeerContainer { tx, address };
    state.lock().unwrap().add_peer(peer);

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let read = rx.read(&mut buf).await;
            let count = match read {
                Ok(c) => c,
                Err(_) => {
                    return;
                }
            };
            if count > 0 {
                // TODO: do something with the contents of buf...
            }
        }
    });
}
