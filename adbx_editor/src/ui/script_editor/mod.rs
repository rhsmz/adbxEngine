pub mod click_handler;
pub mod cursor_position;
pub mod entity_attach;
pub mod file_operations;
pub mod keyboard_input;
pub mod resource;
pub mod syntax_highlight;
pub mod ui_draw;
pub mod validation;

// 公開API
pub use click_handler::handle_script_editor_click;
pub use cursor_position::get_cursor_position;
pub use entity_attach::attach_script_to_entity;
pub use file_operations::load_script_file;
pub use keyboard_input::handle_script_editor_keyboard_input;
pub use resource::{ScriptEditor, ScriptError};
pub use ui_draw::draw_script_editor;
pub use validation::{validate_script, validate_script_system};
