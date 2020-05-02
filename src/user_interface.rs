use cursive::event::Event;
use cursive::menu::{MenuItem, MenuTree};
use cursive::views::{Button, LinearLayout, TextContent, TextView};
use cursive::Cursive;

use std::sync::{Arc, Mutex};

use crate::coffee_network::{
    self,
    ui::{self, ChatView},
};

struct MainUiState {
    chat_view: Arc<Mutex<ChatView>>,
}

impl MainUiState {}

pub async fn start_ui() {
    let network_state = coffee_network::create_network();

    // Initialize the main Cursive controller
    let mut siv = Cursive::default();
    siv.set_fps(5);
    let ui_state = Arc::new(Mutex::new(MainUiState {
        chat_view: Arc::new(Mutex::new(ChatView::new(&mut siv, network_state.clone()))),
    }));

    // Create menu
    {
        let mut file_menu = MenuTree::new();
        file_menu.add_leaf("Quit (Ctrl+Q)", move |s| s.quit());

        let mut network_menu = MenuTree::new();
        {
            let network_state = network_state.clone();
            network_menu.add_leaf("Info", move |s| {
                ui::launch_info_dialog(s, network_state.clone());
            });
        }
        {
            let network_state = network_state;
            network_menu.add_leaf("Connect", move |s| {
                ui::launch_connect_dialog(s, network_state.clone())
            });
        }

        siv.menubar()
            .add_subtree("File", file_menu)
            .add_subtree("Network", network_menu);
        siv.set_autohide_menu(false);
        siv.add_global_callback(Event::CtrlChar('q'), |s| s.quit());
    }

    siv.run();
}
