#[cfg(feature = "ui-cursive")]
mod impl_cursive;
#[cfg(feature = "ui-cursive")]
pub use impl_cursive::start_ui;
// mod main_ui {
// #[path = "../impl_cursive.rs"]
// }

#[cfg(feature = "ui-iced")]
mod impl_iced;
#[cfg(feature = "ui-iced")]
pub use impl_iced::start_ui;
// mod main_ui {
// #[path = "../impl_iced.rs"]
// }

#[cfg(not(any(feature = "ui-cursive", feature = "ui-iced")))]
pub fn start_ui() {}
// mod main_ui {
// }
