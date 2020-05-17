use iced::{
    button, executor, Align, Application, Column, Command, Container, Element, Length, Row,
    Settings, Subscription,
};

use crate::coffee_app::CoffeeAppContext;

#[derive(Debug)]
enum UIView {
    None,
    Network,
    Audio,
    Chat,
}

#[derive(Debug, Clone)]
enum Message {}

struct CoffeeUI {
    context: CoffeeAppContext,
}

impl Application for CoffeeUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            CoffeeUI {
                context: CoffeeAppContext::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Coffee Chat")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center);
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start_ui() {
    CoffeeUI::run(Settings::default())
}
