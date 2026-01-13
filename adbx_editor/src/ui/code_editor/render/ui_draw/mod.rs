pub mod tab_bar;
pub mod editor_area;
pub mod code_lines;
pub mod change_detection;
mod main;

pub use main::draw_code_editor_ui;
pub use tab_bar::draw_tab_bar;
pub use editor_area::{create_editor_area_entity, draw_text_area, draw_text_area_content};
pub use code_lines::{draw_code_lines, draw_cursor};
pub use change_detection::{check_if_update_needed, update_change_detection};
