pub mod ui;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};

use tokio::prelude::*;

pub struct PeerInfo {
    stream: TcpStream,
    address: SocketAddr,
}

#[derive(Clone, Debug)]
pub enum Message {
    Ping,
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    TextChat(SocketAddr, String),
    VoiceChat(SocketAddr, Vec<u8>),
}

struct NetworkStateInner {
    address: SocketAddr,
    // Broadcase for sending messages OUT from the network state
    broadcast_tx: broadcast::Sender<Message>,
    // broadcast_rx: broadcast::Receiver<Message>,
    // MPSC for sending messages INTO the network state
    mpsc_tx: mpsc::Sender<Message>,
    // mpsc_rx: mpsc::Receiver<Message>,
}

impl NetworkStateInner {
    fn get_address(&self) -> SocketAddr {
        self.address
    }

    fn get_to_server_tx(&self) -> mpsc::Sender<Message> {
        self.mpsc_tx.clone()
    }

    fn get_from_server_rx(&self) -> broadcast::Receiver<Message> {
        self.broadcast_tx.subscribe()
    }

    fn handle_message(&mut self, msg: Message) {
        println!("Handling message: {:?}", msg);
        // TODO: do we need to do any processing of the message?

        // Rebroadcast all messages (for now) to all listeners
        if self.broadcast_tx.send(msg).is_err() {
            // TODO: report error to a proper logger
        }
    }

    // TODO: on drop, broadcast a shutdown message and close all channels
}

impl PeerInfo {
    fn get_stream(self) -> TcpStream {
        self.stream
    }

    fn get_address(&self) -> SocketAddr {
        self.address
    }
}

#[derive(Clone)]
pub struct NetworkState {
    inner: Arc<Mutex<NetworkStateInner>>,
}

impl NetworkState {
    fn new_from_inner(inner: NetworkStateInner) -> Self {
        NetworkState {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    fn lock_ref(&self) -> std::sync::MutexGuard<NetworkStateInner> {
        // TODO: handle lock errors (what causes a lock error?)
        self.inner.lock().unwrap()
    }

    fn start_mpsc(&self, mut mrx: mpsc::Receiver<Message>) {
        let state = self.clone();
        tokio::spawn(async move {
            loop {
                if let Some(msg) = mrx.recv().await {
                    state.lock_ref().handle_message(msg);
                }
            }
        });
    }

    fn start_tcp(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            let address = state.lock_ref().get_address();
            let mut listener = TcpListener::bind(address).await.unwrap();
            println!("Listener bound: {:?}", listener.local_addr().unwrap());
            loop {
                let (stream, address) = listener.accept().await.unwrap();
                let peer_info = PeerInfo { stream, address };

                let state = state.clone();
                process_new_peer(state, peer_info);
            }
        });
    }

    pub fn connect_to(&self, address: String) {
        println!("Connecting to: {}", address);
        let state = self.clone();
        tokio::spawn(async move {
            if let Ok(address) = address.parse() {
                match TcpStream::connect(address).await {
                    Ok(stream) => process_new_peer(state, PeerInfo { stream, address }),
                    Err(e) => {
                        // TODO: report an error to a proper logger
                        println!("Server connection failed: {}, {}", address, e);
                    }
                }
            }
        });
    }

    pub fn get_address(&self) -> SocketAddr {
        self.lock_ref().get_address()
    }

    pub fn get_server_sender(&self) -> mpsc::Sender<Message> {
        self.lock_ref().get_to_server_tx()
    }

    pub fn get_broadcast_receiver(&self) -> broadcast::Receiver<Message> {
        self.lock_ref().get_from_server_rx()
    }

    pub fn send_text_message(&self, text: String) {
        let net = self.clone();
        let text = text.clone();
        tokio::spawn(async move {
            let mut sender = net.get_server_sender();
            if let Err(e) = sender
                .send(Message::TextChat(net.get_address(), text.clone()))
                .await
            {
                println!("Error sending text message: {}", e);
            }
            println!("Sent message: {}", text);
        });
    }
}

pub fn create_network() -> NetworkState {
    let (btx, _brx) = broadcast::channel::<Message>(16);
    let (mtx, mrx) = mpsc::channel::<Message>(100);

    let state = NetworkState::new_from_inner(NetworkStateInner {
        address: SocketAddr::from(([0, 0, 0, 0], 22020)),
        broadcast_tx: btx, // clonable, can send across threads
        mpsc_tx: mtx,      // clonable
    });

    // Initiate async loop to receive on MPSC channel
    state.start_mpsc(mrx);

    // Initiate async loop to accept remote connections
    state.start_tcp();

    state
}

fn process_new_peer(state: NetworkState, peer: PeerInfo) {
    println!("Accepting peer: {}", peer.get_address());
    let address = peer.get_address();
    let stream = peer.get_stream();
    let (mut rx, mut tx) = tokio::io::split(stream);

    // TODO: create a UDP connection with this peer for sending/receiving
    // audio data

    // Initiate async peer listen loop
    let mut server_tx = state.get_server_sender();
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
                // TODO: maybe need to wait for a null terminator and
                // break messages apart?
                let text = std::str::from_utf8(&buf[..count]).unwrap().to_owned();
                println!("Message received: {}", text);
                let msg = Message::TextChat(address, text);
                if let Err(_err) = server_tx.send(msg).await {
                    break;
                }
            }
        }

        // Send the server a message that we are disconnecting
        if let Err(_err) = server_tx.send(Message::Disconnect(address)).await {
            return;
        }
    });

    // Initiate async broadcast channel loop with peer sendint
    tokio::spawn(async move {
        // Acquire a receiver for the network state's broadcast channel
        let mut broadcast_rx = state.get_broadcast_receiver();
        while let Ok(msg) = broadcast_rx.recv().await {
            match msg {
                Message::Connect(_) => {}
                Message::Disconnect(_) => {}
                Message::Ping => {}
                Message::TextChat(sender, text) => {
                    if sender != address && tx.write(text.as_bytes()).await.is_err() {
                        break;
                    }
                }
                Message::VoiceChat(sender, _bytes) => {
                    if sender != address {
                        // TODO: encode and send the data over UDP
                    }
                }
            }
        }
    });
}
