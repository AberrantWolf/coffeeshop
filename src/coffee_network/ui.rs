use std::sync::{Arc, Mutex, MutexGuard};

use cursive::traits::*;
use cursive::view::Scrollable;
use cursive::views::{Button, EditView, LinearLayout, Panel, ResizedView, TextContent, TextView};
use cursive::Cursive;

// Internal-only struct for wrapping the Arc<Mutex<...>> around
struct ChatViewInner {
    // TODO: add network state
    chat_content: TextContent, // thread-safe
}

pub struct ChatView {
    inner: Arc<Mutex<ChatViewInner>>,
    // TODO: clients need async access to a chat history, I guess?
    // TODO: need to add clients
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

impl Default for ChatView {
    fn default() -> Self {
        ChatView {
            inner: Arc::new(Mutex::new(ChatViewInner {
                chat_content: TextContent::new("[new chat started]"),
            })),
        }
    }
}

impl ChatView {
    pub fn new_with_cursive(siv: &mut Cursive) -> Self {
        let cv = ChatView::default();
        let user_list =
            Panel::new(LinearLayout::vertical().child(TextView::new("all"))).title("users");
        let chat_view = TextView::new_with_content(cv.get_text_content()).scrollable();
        let typing_box = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(
                EditView::new().with_name("message_text_edit"),
            ))
            .child(Button::new("Send", |s| {
                s.call_on_name("message_text_edit", |view: &mut EditView| {
                    view.set_content("");
                });
            }));
        let horizontal_layout = LinearLayout::horizontal()
            .child(ResizedView::with_min_width(32, user_list))
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
