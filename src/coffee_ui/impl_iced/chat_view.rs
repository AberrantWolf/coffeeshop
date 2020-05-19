use iced::{Command, Container, Element, Length, Row, Subscription, Text};

#[derive(Debug, Clone, Copy)]
pub enum ChatMessage {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatUI {}

impl ChatUI {
    pub fn new() -> Self {
        ChatUI {}
    }

    pub fn update(&mut self, message: ChatMessage) {
        match message {}
    }

    pub fn view(&mut self) -> Element<ChatMessage> {
        Container::new(Text::new("Chat"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
