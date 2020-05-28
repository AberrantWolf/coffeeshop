use tokio::sync::broadcast;
type StdRwLock<T> = std::sync::RwLock<T>;
type StdArc<T> = std::sync::Arc<T>;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ChatEvent {
    Connect,
    Disconnect,
    Text(String),
}

#[derive(Clone)]
pub struct ChatController {
    uuid: Uuid,
    brodacast_tx: broadcast::Sender<ChatEvent>,
    username: String,
}

impl ChatController {
    pub fn new(uuid: Uuid, username: String) -> Self {
        let (brodacast_tx, _) = broadcast::channel::<ChatEvent>(16);
        ChatController {
            uuid,
            brodacast_tx,
            username,
        }
    }

    pub fn get_sender(&self) -> broadcast::Sender<ChatEvent> {
        self.brodacast_tx.clone()
    }

    pub fn get_receiver(&self) -> broadcast::Receiver<ChatEvent> {
        self.brodacast_tx.subscribe()
    }
}
