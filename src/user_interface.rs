use cursive::event::Event;
use cursive::menu::{MenuItem, MenuTree};
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, ResizedView, TextContent, TextView};
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

// TODO: Does this need to be async?
pub async fn start_ui() {
    // Create a starup dialog...
    let mut siv = Cursive::default();
    siv.set_fps(5);

    let start_fn = |s: &mut Cursive| {
        // Get username
        let mut username = "Default User".to_string();
        s.call_on_name("_usr_nick", |v: &mut EditView| {
            username = v.get_content().to_string();
        });
        // Get port number
        let mut port_num = 0u16;
        s.call_on_name("_use_port", |v: &mut EditView| {
            let port_string = v.get_content().to_string();
            if let Ok(p) = port_string.parse::<u16>() {
                port_num = p;
                println!("Parse port number: {}", p);
            } else {
                println!("Couldn't parse port number: {}", port_string);
            }
        });
        let network_state =
            coffee_network::NetworkState::new_with_port_and_username(port_num, username);
        s.pop_layer();
        launch_main_view(s, network_state)
    };
    let username_line = LinearLayout::horizontal()
        .child(TextView::new("Nickname:"))
        .child(ResizedView::with_min_width(
            32,
            EditView::new()
                .content("Default Name")
                .on_submit(move |s, _st| {
                    start_fn(s);
                })
                .with_name("_usr_nick"),
        ));
    let port_line = LinearLayout::horizontal()
        .child(TextView::new("Port Number:"))
        .child(ResizedView::with_min_width(
            32,
            EditView::new()
                .content("0")
                .on_submit(move |s, _st| {
                    start_fn(s);
                })
                .with_name("_use_port"),
        ));

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(username_line)
                .child(port_line)
                .child(
                    LinearLayout::horizontal()
                        .child(Button::new("Quit", |s| {
                            s.quit();
                        }))
                        .child(Button::new("Connect", start_fn)),
                ),
        )
        .title("Coffee Chat Start Options"),
    );

    siv.run();
}

fn launch_main_view(mut siv: &mut Cursive, network_state: coffee_network::NetworkState) {
    // Initialize the main Cursive controller
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
}
