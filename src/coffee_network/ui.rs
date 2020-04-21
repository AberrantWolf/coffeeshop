use cursive::views::{LinearLayout, ListView, Panel};
use cursive::Cursive;

struct ChatClient {
    // TODO: list of clients
// TODO: how to... add clients?
}

#[derive(Default)]
pub struct ChatView {
    // TODO: clients need async access to a chat history, I guess?
// TODO: need to add clients
}

impl ChatView {
    pub fn new(siv: &mut Cursive) -> Self {
        siv.add_layer(Panel::new(LinearLayout::horizontal().child(ListView::new())).title("Chat?"));

        ChatView {}
    }
}
