use iced::{Element, Row, Subscription, Text};

#[derive(Debug)]
enum ChatMessage {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatView {}

impl ChatView {
    pub fn new() -> Self {
        ChatView {}
    }

    fn view(&mut self) -> Element<ChatMessage> {
        Text::new("Chat").into()
    }
}
