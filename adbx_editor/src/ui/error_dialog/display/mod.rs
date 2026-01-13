pub mod buttons;
mod main;
pub mod message_area;
pub mod overlay;
pub mod theme;
pub mod title_bar;

// 公開API
pub use buttons::draw_error_dialog_buttons;
pub use main::draw_error_dialog;
pub use message_area::draw_error_dialog_message_area;
pub use overlay::draw_error_dialog_overlay;
pub use theme::get_error_theme;
pub use title_bar::draw_error_dialog_title_bar;
