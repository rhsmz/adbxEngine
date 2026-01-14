pub mod buttons;
mod main;
pub mod message_area;
pub mod overlay;
pub mod theme;
pub mod title_bar;

// 公開API
pub use main::draw_error_dialog;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use buttons::draw_error_dialog_buttons;
#[allow(unused_imports)]
pub use message_area::draw_error_dialog_message_area;
#[allow(unused_imports)]
pub use overlay::draw_error_dialog_overlay;
#[allow(unused_imports)]
pub use theme::get_error_theme;
#[allow(unused_imports)]
pub use title_bar::draw_error_dialog_title_bar;
