use crate::coffee_app::CoffeeAppContext;
use iced::{
    button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row,
    Scrollable, Subscription, Text, TextInput,
};

#[derive(Debug, Clone)]
pub enum NetUIMessage {
    PortNumberUpdated(String),
    StartNetwork,
}

#[derive(Debug, Clone)]
pub struct StartupState {
    port_number_state: text_input::State,
    port_number_value: String,
    start_button_state: button::State,
}

impl StartupState {
    pub fn new() -> Self {
        StartupState {
            port_number_state: text_input::State::new(),
            port_number_value: String::new(),
            start_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: NetUIMessage, ctx: &mut CoffeeAppContext) {
        match message {
            NetUIMessage::PortNumberUpdated(val) => {
                let to_num = val.parse::<u16>();
                if let Ok(num) = to_num {
                    self.port_number_value = num.to_string();
                } else if val.is_empty() {
                    self.port_number_value = val;
                }
            }
            NetUIMessage::StartNetwork => {}
        }
    }

    pub fn subscription(&self) -> Subscription<NetUIMessage> {
        Subscription::none()
    }

    pub fn view(&mut self, ctx: &CoffeeAppContext) -> Element<NetUIMessage> {
        let port_label = Text::new("Port");
        let port_input = TextInput::new(
            &mut self.port_number_state,
            "IP port number...",
            &mut self.port_number_value,
            NetUIMessage::PortNumberUpdated,
        )
        .padding(4)
        .on_submit(NetUIMessage::StartNetwork);

        let start_btn = Button::new(&mut self.start_button_state, Text::new("Accept"))
            .on_press(NetUIMessage::StartNetwork);
        Row::new()
            .push(port_label)
            .push(port_input)
            .push(start_btn)
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct RunningState {}

#[derive(Debug, Clone)]
pub enum NetUI {
    Startup(StartupState),
    Running(RunningState),
}

impl NetUI {
    pub fn new() -> Self {
        NetUI::Startup(StartupState {
            port_number_state: text_input::State::new(),
            port_number_value: String::new(),
            start_button_state: button::State::new(),
        })
    }

    pub fn update(&mut self, message: NetUIMessage, ctx: &mut CoffeeAppContext) {
        match self {
            NetUI::Startup(startup) => startup.update(message, ctx),
            NetUI::Running(_) => match message {
                NetUIMessage::PortNumberUpdated(_) => {}
                NetUIMessage::StartNetwork => {}
            },
        }
    }

    pub fn subscription(&self) -> Subscription<NetUIMessage> {
        match self {
            NetUI::Startup(_) => Subscription::none(),
            NetUI::Running(_) => Subscription::none(),
        }
    }

    pub fn view(&mut self, ctx: &CoffeeAppContext) -> Element<NetUIMessage> {
        let label = Text::new("Network");

        let main_view: Element<_> = match self {
            NetUI::Startup(startup) => startup.view(ctx),
            NetUI::Running(_) => Column::new().into(),
        };

        let final_view = Column::new()
            .align_items(Align::Center)
            .push(label)
            .push(main_view);

        Container::new(final_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
