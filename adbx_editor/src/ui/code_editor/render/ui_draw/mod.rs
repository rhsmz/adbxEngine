pub mod change_detection;
pub mod code_lines;
pub mod editor_area;
mod main;
pub mod tab_bar;

pub use main::draw_code_editor_ui;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use change_detection::{check_if_update_needed, update_change_detection};
#[allow(unused_imports)]
pub use code_lines::{draw_code_lines, draw_cursor};
#[allow(unused_imports)]
pub use editor_area::{create_editor_area_entity, draw_text_area, draw_text_area_content};
#[allow(unused_imports)]
pub use tab_bar::draw_tab_bar;
