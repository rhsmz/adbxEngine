pub mod resource;
pub mod syntax_highlight;
pub mod validation;
pub mod ui_draw;
pub mod click_handler;
pub mod file_operations;
pub mod entity_attach;
pub mod keyboard_input;
pub mod cursor_position;

// 公開API
pub use resource::{ScriptEditor, ScriptError};
pub use validation::{validate_script, validate_script_system};
pub use ui_draw::draw_script_editor;
pub use click_handler::handle_script_editor_click;
pub use file_operations::load_script_file;
pub use entity_attach::attach_script_to_entity;
pub use keyboard_input::handle_script_editor_keyboard_input;
pub use cursor_position::get_cursor_position;
