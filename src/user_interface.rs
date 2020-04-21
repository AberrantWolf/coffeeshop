use cursive::views::{Button, LinearLayout, TextContent, TextView};
use cursive::Cursive;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use crate::coffee_network::ui::ChatView;

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

    let app_state = Arc::new(Mutex::new(MainUiState {
        chat_view: Arc::new(Mutex::new(ChatView::new(&mut siv))),
        val: 0usize,
        connect_text: TextContent::new("<connect text>"),
        port_text: TextContent::new("<ip address>"),
        counter: TextContent::new("0"),
    }));
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], 0));

    {
        let app_state = app_state.clone();
        tokio::spawn(async move {
            let mut listener = TcpListener::bind(&listen_addr).await.unwrap();
            app_state
                .lock()
                .unwrap()
                .port_text
                .set_content(format!("{}", listener.local_addr().unwrap()));
            loop {
                let (_stream, addr) = listener.accept().await.unwrap();
                let state = app_state.lock().unwrap();
                state.connect_text.set_content(format!("{}", addr));
            }
        });
    }

    siv.set_user_data(app_state.clone());
    {
        let app_state = app_state.lock().unwrap();
        siv.add_layer(
            LinearLayout::vertical()
                .child(TextView::new_with_content(app_state.connect_text.clone()))
                .child(TextView::new_with_content(app_state.port_text.clone()))
                .child(TextView::new_with_content(app_state.counter.clone()))
                .child(Button::new("Increment", |s| {
                    s.with_user_data(|data: &mut Arc<Mutex<MainUiState>>| {
                        let mut ui_state = data.lock().unwrap();
                        ui_state.val += 1;
                        ui_state.counter.set_content(format!("{}", ui_state.val));
                    });
                })),
        );
    }
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
