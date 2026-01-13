pub mod theme;
pub mod overlay;
pub mod title_bar;
pub mod message_area;
pub mod buttons;
mod main;

// 公開API
pub use main::draw_error_dialog;
pub use theme::get_error_theme;
pub use overlay::draw_error_dialog_overlay;
pub use title_bar::draw_error_dialog_title_bar;
pub use message_area::draw_error_dialog_message_area;
pub use buttons::draw_error_dialog_buttons;
