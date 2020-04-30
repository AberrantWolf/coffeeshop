use cursive::event::Event;
use cursive::menu::{MenuItem, MenuTree};
use cursive::views::{Button, LinearLayout, TextContent, TextView};
use cursive::Cursive;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use crate::coffee_network::{self, ui::ChatView};

struct MainUiState {
    chat_view: Arc<Mutex<ChatView>>,
    val: usize,
    connect_text: TextContent,
    port_text: TextContent,
    counter: TextContent,
}

impl MainUiState {}

pub async fn start_ui() {
    let mut siv = Cursive::default();

    // Create menu
    siv.menubar()
        .add_subtree(
            "File",
            MenuTree::new().leaf("Quit (Ctrl+Q)", move |s| s.quit()),
        )
        .add_subtree("Network", MenuTree::new().leaf("Connect", move |s| {}));
    siv.set_autohide_menu(false);

    let app_state = Arc::new(Mutex::new(MainUiState {
        chat_view: Arc::new(Mutex::new(ChatView::new_with_cursive(&mut siv))),
        val: 0usize,
        connect_text: TextContent::new("<connect text>"),
        port_text: TextContent::new("<ip address>"),
        counter: TextContent::new("0"),
    }));

    let network_state = coffee_network::create_network();
    coffee_network::start_server(network_state.clone(), Box::new(move |peer| {}));

    // siv.set_user_data(app_state.clone());
    // {
    //     let app_state = app_state.lock().unwrap();
    //     siv.add_layer(
    //         LinearLayout::vertical()
    //             .child(TextView::new_with_content(app_state.connect_text.clone()))
    //             .child(TextView::new_with_content(app_state.port_text.clone()))
    //             .child(TextView::new_with_content(app_state.counter.clone()))
    //             .child(Button::new("Increment", |s| {
    //                 s.with_user_data(|data: &mut Arc<Mutex<MainUiState>>| {
    //                     let mut ui_state = data.lock().unwrap();
    //                     ui_state.val += 1;
    //                     ui_state.counter.set_content(format!("{}", ui_state.val));
    //                 });
    //             })),
    //     );
    // }
    siv.add_global_callback(Event::CtrlChar('q'), |s| s.quit());

    siv.run();
}
