use std::sync::{Arc, Mutex, MutexGuard};

use cursive::traits::*;
use cursive::view::Scrollable;
use cursive::views::{Button, EditView, LinearLayout, Panel, ResizedView, TextContent, TextView};
use cursive::Cursive;

use crate::coffee_network::{Message, NetworkState};

// Internal-only struct for wrapping the Arc<Mutex<...>> around
struct ChatViewInner {
    // TODO: add network state
    chat_content: TextContent, // thread-safe
}

#[derive(Clone)]
pub struct ChatView {
    inner: Arc<Mutex<ChatViewInner>>,
}

impl ChatView {
    fn lock_ref(&self) -> MutexGuard<ChatViewInner> {
        // TODO: handle lock errors (what causes a lock error?)
        self.inner.lock().unwrap()
    }
    fn get_text_content(&self) -> TextContent {
        self.lock_ref().chat_content.clone()
    }
}

impl ChatView {
    pub fn new(siv: &mut Cursive, net: NetworkState) -> Self {
        let cv = ChatView {
            inner: Arc::new(Mutex::new(ChatViewInner {
                chat_content: TextContent::new("[new chat started]\n"),
            })),
        };

        {
            let cv = cv.clone();
            let mut receiver = net.get_broadcast_receiver();
            tokio::spawn(async move {
                loop {
                    match receiver.recv().await {
                        Ok(msg) => match msg {
                            Message::TextChat(_sender, text) => {
                                // TODO: check sender and prepend username or something
                                cv.get_text_content().append(format!("{}\n", text));
                            }
                            _ => unimplemented!(),
                        },
                        Err(e) => {
                            // TODO: Log error
                        }
                    }
                }
            });
        }

        let user_list_panel =
            Panel::new(LinearLayout::vertical().child(TextView::new("all"))).title("users");
        let chat_view = TextView::new_with_content(cv.get_text_content()).scrollable();
        let typing_box = {
            let edit_view = {
                let net = net.clone();
                ResizedView::with_full_width(
                    EditView::new()
                        .on_submit_mut(move |s, _text| {
                            let net = net.clone();
                            s.call_on_name("message_text_edit", move |view: &mut EditView| {
                                println!("Sending!");
                                let text = view.get_content().to_string();
                                view.set_content("");
                                net.send_text_message(text);
                            });
                        })
                        .with_name("message_text_edit"),
                )
            };
            let submit_btn = {
                let net = net.clone();
                Button::new("Send", move |s| {
                    let net = net.clone();
                    s.call_on_name("message_text_edit", move |view: &mut EditView| {
                        println!("Sending!");
                        let text = view.get_content().to_string();
                        view.set_content("");
                        net.send_text_message(text);
                    });
                })
            };
            LinearLayout::horizontal()
                .child(edit_view)
                .child(submit_btn)
        };
        let horizontal_layout = LinearLayout::horizontal()
            .child(ResizedView::with_min_width(32, user_list_panel))
            .child(ResizedView::with_full_screen(
                Panel::new(chat_view).title("Messages"),
            ));
        let vertical_layout = LinearLayout::vertical()
            .child(ResizedView::with_full_height(horizontal_layout))
            .child(typing_box);
        siv.add_fullscreen_layer(ResizedView::with_full_screen(
            Panel::new(vertical_layout).title("Chat"),
        ));
        cv
    }
}
