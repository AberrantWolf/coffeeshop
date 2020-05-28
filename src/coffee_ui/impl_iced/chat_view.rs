mod chat_recipe;

use chat_recipe::chat_subscription;

use iced::{
    button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row,
    Scrollable, Subscription, Text, TextInput,
};

// use iced_native::futures::StreamExt;

use crate::coffee_app::CoffeeAppContext;
use crate::coffee_chat::ChatEvent;

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum ChatMessage {
    UsernameUpdated(String),
    ChatStringUpdate(String),
    StartChat,
    // Event(Instant),
    Event(ChatEvent),
    SendChat,
}

//=====================================================
// Startup State
//
#[derive(Debug, Clone)]
pub struct StartupState {
    username_state: text_input::State,
    username_value: String,
    start_button_state: button::State,
}

impl StartupState {
    fn new() -> Self {
        StartupState {
            username_state: text_input::State::new(),
            username_value: String::from(""),
            start_button_state: button::State::new(),
        }
    }

    fn update(&mut self, message: &ChatMessage, ctx: &mut CoffeeAppContext) {
        match message {
            ChatMessage::UsernameUpdated(name) => self.username_value = name.clone(),
            ChatMessage::StartChat => {
                ctx.start_chat(self.username_value.clone());
            }
            _ => {}
        }
    }

    fn subscription(&self) -> Subscription<ChatMessage> {
        Subscription::none()
    }
    fn view(&mut self, ctx: &CoffeeAppContext) -> Element<ChatMessage> {
        let username_label = Text::new("Username");
        let username_input = TextInput::new(
            &mut self.username_state,
            "Write your nickname...",
            &self.username_value,
            ChatMessage::UsernameUpdated,
        )
        .padding(4)
        .on_submit(ChatMessage::StartChat);
        let username_row = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(username_label)
            .push(username_input);

        let start_btn = Button::new(&mut self.start_button_state, Text::new("Accept"))
            .on_press(ChatMessage::StartChat);

        Column::new()
            .spacing(10)
            .padding(20)
            .align_items(Align::Center)
            .push(username_row)
            .push(start_btn)
            .into()
    }
}

//=====================================================
// Runnint State
//
#[derive(Debug, Clone)]
pub struct RunningState {
    broadcast_tx: broadcast::Sender<ChatEvent>,
    user_scroll_state: scrollable::State,
    chat_scroll_state: scrollable::State,
    chat_input_state: text_input::State,
    chat_input_value: String,
    send_button_state: button::State,
    messages: Vec<String>, // TODO: message struct with sender, timestamp, etc}
}

impl RunningState {
    fn new(ctx: &CoffeeAppContext) -> Self {
        RunningState {
            broadcast_tx: ctx.chat().as_ref().unwrap().get_sender(),
            user_scroll_state: scrollable::State::new(),
            chat_scroll_state: scrollable::State::new(),
            chat_input_state: text_input::State::new(),
            chat_input_value: String::new(),
            send_button_state: button::State::new(),
            messages: vec![],
        }
    }

    fn update(&mut self, message: &ChatMessage, ctx: &mut CoffeeAppContext) {
        match message {
            ChatMessage::ChatStringUpdate(s) => self.chat_input_value = s.clone(),
            ChatMessage::SendChat => {
                self.broadcast_tx
                    .send(ChatEvent::Text(self.chat_input_value.clone()));
                self.chat_input_value = String::new();
            }
            ChatMessage::Event(evt) => match evt {
                ChatEvent::Connect => self.messages.push(String::from("Connection")),
                ChatEvent::Disconnect => self.messages.push(String::from("Disconection")),
                ChatEvent::Text(s) => {
                    // chat_scroll_state.scroll_to()
                    self.messages.push(s.clone())
                }
            },
            _ => {}
        }
    }

    pub fn subscription(&self) -> Subscription<ChatMessage> {
        chat_subscription(&self.broadcast_tx)
    }
    fn view(&mut self, ctx: &CoffeeAppContext) -> Element<ChatMessage> {
        let user_list = Scrollable::new(&mut self.user_scroll_state).width(Length::FillPortion(1));
        // TODO: Add users...
        let messages_col: Column<ChatMessage> = Column::with_children(
            self.messages
                .iter()
                .map(|m| Text::new(m.clone()).into())
                .collect(),
        )
        .spacing(4);
        let mut message_list = Scrollable::new(&mut self.chat_scroll_state)
            .width(Length::FillPortion(5))
            .push(messages_col);

        let main_row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(user_list)
            .push(message_list);

        let input_text = TextInput::new(
            &mut self.chat_input_state,
            "Type a message...",
            &self.chat_input_value,
            ChatMessage::ChatStringUpdate,
        )
        .padding(4)
        .width(Length::Fill)
        .on_submit(ChatMessage::SendChat);
        let send_button = Button::new(&mut self.send_button_state, Text::new("Send"))
            .on_press(ChatMessage::SendChat);
        let input_row = Row::new()
            .width(Length::Fill)
            .push(input_text)
            .push(send_button);

        Column::new().push(main_row).push(input_row).into()
    }
}

//=====================================================
// Main Chat UI
//
#[derive(Debug, Clone)]
pub enum ChatUI {
    Startup(StartupState),
    Running(RunningState),
}

impl ChatUI {
    pub fn new() -> Self {
        ChatUI::Startup(StartupState::new())
    }

    fn new_running(ctx: &CoffeeAppContext) -> Self {
        ChatUI::Running(RunningState::new(ctx))
    }

    pub fn update(&mut self, message: ChatMessage, ctx: &mut CoffeeAppContext) {
        match self {
            ChatUI::Startup(state) => {
                state.update(&message, ctx);

                // The state can't really handle CHANGING states very well...
                if let ChatMessage::StartChat = message {
                    *self = Self::new_running(&ctx);
                }
            }
            ChatUI::Running(state) => state.update(&message, ctx),
        }
    }

    pub fn subscription(&self) -> Subscription<ChatMessage> {
        match self {
            ChatUI::Startup(state) => state.subscription(),
            ChatUI::Running(state) => state.subscription(),
        }
    }

    pub fn view(&mut self, ctx: &CoffeeAppContext) -> Element<ChatMessage> {
        let label = Text::new("Chat");

        let main_view: Element<_> = match self {
            ChatUI::Startup(state) => state.view(ctx),
            ChatUI::Running(state) => state.view(ctx),
        };

        let final_view = Column::new().push(label).push(main_view);

        Container::new(final_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
