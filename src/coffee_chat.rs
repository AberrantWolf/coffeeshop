// Chat module for handling chat state and messages and queries

use tokio::sync::{broadcast, mpsc};
type StdRwLock<T> = std::sync::RwLock<T>;
type StdArc<T> = std::sync::Arc<T>;

#[derive(Clone)]
pub enum ChatMessage {
    Connect,
    Disconnect,
    Text(String),
}

struct ChatControllerInner {
    brodacast_tx: broadcast::Sender<ChatMessage>,
}

#[derive(Clone)]
pub struct ChatController {
    inner: StdArc<StdRwLock<ChatControllerInner>>,
}

impl ChatController {
    pub fn new() -> Self {
        let (brodacast_tx, _) = broadcast::channel::<ChatMessage>(16);
        ChatController {
            inner: StdArc::new(StdRwLock::new(ChatControllerInner { brodacast_tx })),
        }
    }
}
