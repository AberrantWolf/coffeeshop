pub mod testmod {}

use cursive::event::Event;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, ResizedView, TextView};
use cursive::Cursive;

use std::sync::{Arc, Mutex};

use crate::coffee_app::CoffeeAppContext;
use chat_view::ChatView;

struct MainUiState {
    chat_view: Arc<Mutex<ChatView>>,
}

impl MainUiState {}

pub fn start_ui() {
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
        s.pop_layer();
        // Construct the main app context binding and launch the main UI
        let coffee_app = CoffeeAppContext::construct(port_num, username);
        launch_main_view(s, coffee_app);
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

fn launch_main_view(mut siv: &mut Cursive, coffee_app: CoffeeAppContext) {
    // Initialize the main Cursive controller
    let ui_state = Arc::new(Mutex::new(MainUiState {
        chat_view: Arc::new(Mutex::new(ChatView::new(
            &mut siv,
            coffee_app.get_net_controller().clone(),
        ))),
    }));

    // Create menu
    {
        let mut file_menu = MenuTree::new();
        file_menu.add_leaf("Quit (Ctrl+Q)", move |s| s.quit());

        let mut network_menu = MenuTree::new();
        {
            let net = coffee_app.get_net_controller().clone();
            network_menu.add_leaf("Info", move |s| {
                ui::launch_info_dialog(s, net.clone());
            });
        }
        {
            let net = coffee_app.get_net_controller().clone();
            network_menu.add_leaf("Connect", move |s| {
                ui::launch_connect_dialog(s, net.clone())
            });
        }

        let mut audio_menu = MenuTree::new();
        {
            let audio = coffee_app.get_audio_controller().clone();
            audio_menu.add_leaf("Print Input", move |_s| {
                println!("Input device: {:?}", audio.get_input_config())
            });
        }
        {
            let audio = coffee_app.get_audio_controller().clone();
            audio_menu.add_leaf("Print Output", move |_s| {
                println!("Output device: {:?}", audio.get_output_config())
            });
        }

        siv.menubar()
            .add_subtree("File", file_menu)
            .add_subtree("Network", network_menu)
            .add_subtree("Audio", audio_menu);
        siv.set_autohide_menu(false);
        siv.add_global_callback(Event::CtrlChar('q'), |s| s.quit());
    }
}
