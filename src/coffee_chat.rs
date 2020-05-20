use tokio::sync::{broadcast, mpsc};
type StdRwLock<T> = std::sync::RwLock<T>;
type StdArc<T> = std::sync::Arc<T>;

use uuid::Uuid;

#[derive(Clone)]
pub enum ChatMessage {
    Connect,
    Disconnect,
    Text(String),
}

// struct ChatControllerInner {}

#[derive(Clone)]
pub struct ChatController {
    local_id: Uuid,
    brodacast_tx: broadcast::Sender<ChatMessage>,
    username: String,
}

impl ChatController {
    pub fn new(local_id: Uuid, username: String) -> Self {
        let (brodacast_tx, _) = broadcast::channel::<ChatMessage>(16);
        ChatController {
            local_id,
            brodacast_tx,
            username,
        }
    }
}
