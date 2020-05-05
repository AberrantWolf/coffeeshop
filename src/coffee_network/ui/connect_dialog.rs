use crate::coffee_network::NetworkState;
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, ResizedView, TextContent, TextView};
use cursive::Cursive;

pub fn launch_info_dialog(siv: &mut Cursive, net: NetworkState) {
    let address_content = TextContent::new("[loading]");
    tokio::spawn({
        let content = address_content.clone();
        async move {
            let address_text = format!("{}", net.get_address().await);
            content.set_content(address_text);
        }
    });
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new("Address:"))
                .child(TextView::new_with_content(address_content)),
        )
        .dismiss_button("Okay"),
    );
}

pub fn launch_connect_dialog(siv: &mut Cursive, net: NetworkState) {
    let address_layout = {
        let net = net.clone();
        LinearLayout::horizontal()
            .child(TextView::new("Address:"))
            .child(ResizedView::with_min_width(
                32,
                EditView::new()
                    .on_submit(move |s, st| {
                        net.connect_to(st.to_string());
                        s.pop_layer();
                    })
                    .with_name("addr"),
            ))
    };

    let main_layout = {
        let net = net; // clone if adding anything below
        LinearLayout::vertical().child(address_layout).child(
            LinearLayout::horizontal()
                .child(Button::new("Cancel", |s| {
                    s.pop_layer();
                }))
                .child(Button::new("Connect", move |s| {
                    s.call_on_name("addr", |view: &mut EditView| {
                        let addr = view.get_content().to_string();
                        net.connect_to(addr);
                    });
                    s.pop_layer();
                })),
        )
    };

    siv.add_layer(Dialog::around(main_layout));
}
