pub mod ui;

mod peer;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, RwLock};

use uuid::Uuid;

use self::peer::Peer;

#[derive(Clone, Debug)]
pub enum Message {
    _Connect(Uuid),
    Disconnect(Uuid),
    TextChat(Uuid, String),
    _VoiceChat(Uuid, Vec<u8>),
}

#[derive(Debug)]
struct NetworkControllerPrivate {
    address: SocketAddr,
    local_id: Uuid,
    local_nick: String,
    // Broadcase for sending messages OUT from the network state
    broadcast_tx: broadcast::Sender<Message>,
    // MPSC for sending messages INTO the network state
    mpsc_tx: mpsc::Sender<Message>,
    peers: Vec<Peer>,
}

#[derive(Clone, Debug)]
pub struct NetworkController {
    inner: Arc<RwLock<NetworkControllerPrivate>>,
}

impl NetworkController {
    pub fn new_with_port_and_username(port_num: u16, username: String) -> Self {
        let (btx, _brx) = broadcast::channel::<Message>(16);
        let (mtx, mrx) = mpsc::channel::<Message>(100);
        let state = NetworkController {
            inner: Arc::new(RwLock::new(NetworkControllerPrivate {
                address: SocketAddr::from(([0, 0, 0, 0], port_num)),
                local_id: Uuid::new_v4(),
                local_nick: username,
                broadcast_tx: btx,
                mpsc_tx: mtx,
                peers: vec![],
            })),
        };

        // Initiate async loop to receive on MPSC channel
        state.start_mpsc(mrx);

        // Initiate async loop to accept remote connections
        state.start_tcp_server();

        state
    }

    async fn add_peer(&mut self, peer: Peer) {
        self.inner.write().await.peers.push(peer)
    }

    async fn handle_message(&mut self, msg: Message) {
        println!("Handling message: {:?}", msg);
        println!(
            "Number of receivers: {}",
            self.inner.read().await.broadcast_tx.receiver_count()
        );
        // TODO: do we need to do any processing of the message?
        // match msg {
        //     Message::_Connect(_) => {}
        //     Message::Disconnect(_) => {}
        //     Message::TextChat(sender, _) => {
        //         if sender == self.get_local_id().await {
        //             return;
        //         }
        //     }
        //     Message::_VoiceChat(_, _) => {}
        // }

        // Rebroadcast all messages (for now) to all listeners
        if self
            .inner
            .read()
            .await
            .broadcast_tx
            .send(msg.clone())
            .is_err()
        {
            // TODO: report error to a proper logger
            println!("Error broadcasting message from server");
        }
        println!("Message rebroadcast: {:?}", msg);
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

    fn start_tcp_server(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            let address = state.get_address().await;
            let mut listener = TcpListener::bind(address).await.unwrap();
            match listener.local_addr() {
                Ok(address) => {
                    state.set_address(address).await;
                    println!("TCP Listener bound: {:?}", address);
                }
                Err(e) => {
                    println!("Error binding TCP listener: {}", e);
                    return;
                }
            }
            loop {
                let (stream, address) = listener.accept().await.unwrap();
                let state = state.clone();

                println!("Accepting incoming peer: {}", address);
                process_new_peer(state, stream);
            }
        });
    }

    pub fn connect_to(&self, address_string: String) {
        println!("Connecting to: {}", address_string);
        let state = self.clone();
        tokio::spawn(async move {
            if let Ok(address) = address_string.parse::<SocketAddr>() {
                match TcpStream::connect(address).await {
                    Ok(stream) => {
                        process_new_peer(state.clone(), stream);
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

    pub async fn get_local_id(&self) -> Uuid {
        self.inner.read().await.local_id
    }

    pub async fn get_local_nick(&self) -> String {
        self.inner.read().await.local_nick.clone()
    }

    pub async fn get_address(&self) -> SocketAddr {
        self.inner.read().await.address
    }
    async fn set_address(&self, address: SocketAddr) {
        self.inner.write().await.address = address;
    }

    pub async fn get_server_sender(&self) -> mpsc::Sender<Message> {
        self.inner.read().await.mpsc_tx.clone()
    }

    pub async fn get_broadcast_receiver(&self) -> broadcast::Receiver<Message> {
        self.inner.read().await.broadcast_tx.subscribe()
    }

    pub fn send_text_message(&self, text: String) {
        let net = self.clone();
        tokio::spawn(async move {
            let mut sender = net.get_server_sender().await;
            if let Err(e) = sender
                .send(Message::TextChat(net.get_local_id().await, text.clone()))
                .await
            {
                println!("Error sending text message: {}", e);
            }
            println!("Sent message: {}", text);
        });
    }
}

fn process_new_peer(mut net: NetworkController, stream: TcpStream) {
    tokio::spawn(async move {
        let peer = match Peer::new(stream, net.clone()).await {
            Ok(peer) => peer,
            Err(_e) => {
                // TODO: Log e somewhere
                return;
            }
        };
        net.add_peer(peer.clone()).await;
    });
}
