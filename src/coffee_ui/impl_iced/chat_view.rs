use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, Length, Row,
    Subscription, Text, TextInput,
};

use crate::coffee_app::CoffeeAppContext;

#[derive(Debug, Clone)]
pub enum ChatMessage {
    UsernameUpdated(String),
    StartChat,
}

#[derive(Debug, Clone)]
pub enum ChatUI {
    Startup {
        username_state: text_input::State,
        username_value: String,
        start_button_state: button::State,
    },
    Running {},
}

impl ChatUI {
    pub fn new() -> Self {
        ChatUI::Startup {
            username_state: text_input::State::new(),
            username_value: String::from(""),
            start_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: ChatMessage, ctx: &mut CoffeeAppContext) {
        match self {
            &mut ChatUI::Startup {
                ref mut username_value,
                ..
            } => match message {
                ChatMessage::UsernameUpdated(name) => *username_value = name,
                ChatMessage::StartChat => {
                    ctx.start_chat(username_value.clone());
                    *self = ChatUI::Running {}
                }
            },
            ChatUI::Running {} => {}
        }
    }

    pub fn view(&mut self, ctx: &CoffeeAppContext) -> Element<ChatMessage> {
        let label = Text::new("Chat");

        let main_view: Element<_> = match self {
            ChatUI::Startup {
                ref mut username_state,
                username_value,
                ref mut start_button_state,
            } => {
                let username_label = Text::new("Username");
                let username_input = TextInput::new(
                    username_state,
                    "Write your nickname...",
                    &username_value,
                    ChatMessage::UsernameUpdated,
                );
                let username_row = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(username_label)
                    .push(username_input);

                let start_btn = Button::new(start_button_state, Text::new("Accept"))
                    .on_press(ChatMessage::StartChat);

                Column::new()
                    .spacing(10)
                    .padding(20)
                    .align_items(Align::Center)
                    .push(label)
                    .push(username_row)
                    .push(start_btn)
                    .into()
            }
            ChatUI::Running { .. } => Text::new("Running").into(),
        };

        Container::new(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
