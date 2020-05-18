mod chat_view;

use iced::{
    button, executor, Align, Application, Column, Command, Container, Element, Length, Radio, Row,
    Settings, Subscription,
};

use crate::coffee_app::CoffeeAppContext;

use chat_view::ChatView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UIView {
    Network(),
    Audio(),
    Chat(ChatView),
}

#[derive(Debug, Clone, Copy)]
enum CoffeeUIMessage {
    ChangeView(UIView),
}

struct CoffeeUI {
    context: CoffeeAppContext,
    current_view: UIView,
}

impl Application for CoffeeUI {
    type Executor = executor::Default;
    type Message = CoffeeUIMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<CoffeeUIMessage>) {
        (
            CoffeeUI {
                context: CoffeeAppContext::new(),
                current_view: UIView::Network(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Coffee Chat")
    }

    fn update(&mut self, message: CoffeeUIMessage) -> Command<CoffeeUIMessage> {
        match message {
            CoffeeUIMessage::ChangeView(v) => self.current_view = v,
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<CoffeeUIMessage> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<CoffeeUIMessage> {
        let network_radio = Radio::new(
            UIView::Network(),
            "Network",
            Some(self.current_view),
            CoffeeUIMessage::ChangeView,
        );
        let chat_radio = Radio::new(
            UIView::Chat(ChatView::new()),
            "Chat",
            Some(self.current_view),
            CoffeeUIMessage::ChangeView,
        );
        let audio_radio = Radio::new(
            UIView::Audio(),
            "Audio",
            Some(self.current_view),
            CoffeeUIMessage::ChangeView,
        );

        let mode_select_row = Row::new()
            .spacing(20)
            .padding(10)
            .align_items(Align::Center)
            .push(network_radio)
            .push(chat_radio)
            .push(audio_radio);

        let content = Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(mode_select_row);

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
