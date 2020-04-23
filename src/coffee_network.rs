pub mod ui;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

pub struct PeerInfo {
    stream: TcpStream,
    address: SocketAddr,
}

#[derive(Default)]
pub struct NetworkStateInner {
    address: Option<SocketAddr>,
    peers: Vec<PeerInfo>,
}

impl NetworkStateInner {
    fn add_peer(&mut self, info: PeerInfo) {
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

    fn get_stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    fn get_address(&self) -> &SocketAddr {
        &self.address
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
    state.lock().unwrap().add_peer(peer);

    tokio::spawn(async move {
        loop {
            // Listen for incoming messages
        }
    });
}
