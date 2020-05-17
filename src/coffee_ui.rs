#[cfg(feature = "ui-cursive")]
mod main_ui {
    #[path = "../impl_cursive.rs"]
    mod impl_cursive;
    pub use impl_cursive::*;
}

#[cfg(feature = "ui-iced")]
mod main_ui {
    #[path = "../impl_iced.rs"]
    mod impl_iced;
    pub use impl_iced::*;
}

#[cfg(not(any(feature = "ui-cursive", feature = "ui-iced")))]
mod main_ui {
    pub fn start_ui() {}
}

pub use main_ui::*;
