mod chat_view;

use iced::{
    executor, Align, Application, Column, Command, Container, Element, Length, Radio, Row,
    Settings, Subscription, Text,
};

use crate::coffee_app::CoffeeAppContext;

use chat_view::{ChatMessage, ChatUI};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UIView {
    Network,
    Audio,
    Chat,
}

#[derive(Debug, Clone, Copy)]
enum CoffeeUIMessage {
    ChangeView(UIView),
    ChatUIMessage(ChatMessage),
}

struct CoffeeUI {
    context: CoffeeAppContext,
    current_view: UIView,
    chat_ui: ChatUI,
}

impl Application for CoffeeUI {
    type Executor = executor::Default;
    type Message = CoffeeUIMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<CoffeeUIMessage>) {
        (
            CoffeeUI {
                context: CoffeeAppContext::new(),
                current_view: UIView::Network,
                chat_ui: ChatUI::new(),
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
            CoffeeUIMessage::ChatUIMessage(_cm) => unimplemented!(),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<CoffeeUIMessage> {
        // TODO: Create a subscription for chat messages that listens on
        // the broadcast channel for them to come in and sends chat message
        // update enums...
        Subscription::none()
    }

    fn view(&mut self) -> Element<CoffeeUIMessage> {
        // let CoffeeUI { current_view, .. } = self;

        let network_radio = Radio::new(
            UIView::Network,
            "Network",
            Some(self.current_view),
            CoffeeUIMessage::ChangeView,
        );
        let chat_radio = Radio::new(
            UIView::Chat,
            "Chat",
            Some(self.current_view),
            CoffeeUIMessage::ChangeView,
        );
        let audio_radio = Radio::new(
            UIView::Audio,
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

        let main_view = match self.current_view {
            UIView::Audio => Text::new("Audio").into(),
            UIView::Chat => self.chat_ui.view().map(CoffeeUIMessage::ChatUIMessage),
            UIView::Network => Text::new("Network").into(),
        };

        let content = Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(mode_select_row)
            .push(main_view);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .into()
    }
}

pub fn start_ui() {
    CoffeeUI::run(Settings::default())
}
