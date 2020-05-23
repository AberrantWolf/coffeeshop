use iced::Subscription;
use iced_native::futures;
use tokio::stream::StreamExt;
use tokio::sync::broadcast;

use super::ChatMessage;
use crate::coffee_chat::ChatEvent;

pub fn chat_subscription(broadcast_tx: &broadcast::Sender<ChatEvent>) -> Subscription<ChatMessage> {
    Subscription::from_recipe(ChatListener {
        broadcast_tx: broadcast_tx.clone(),
    })
}

struct ChatListener {
    broadcast_tx: broadcast::Sender<ChatEvent>,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for ChatListener
where
    H: std::hash::Hasher,
{
    type Output = ChatMessage;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(
            self.broadcast_tx
                .subscribe()
                .into_stream()
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .map(ChatMessage::Event),
        )
    }
}
