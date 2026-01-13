pub mod character_input;
pub mod editing_operations;
pub mod navigation;
pub mod shortcuts;
mod main;

pub use main::handle_script_editor_keyboard_input;
pub use character_input::insert_text_at_cursor_script;
pub use editing_operations::{handle_backspace_script_editor, handle_delete_script_editor, undo_edit, redo_edit};
pub use navigation::{handle_arrow_left_script_editor, handle_arrow_right_script_editor};
pub use shortcuts::{copy_selection, paste_text, cut_selection, select_all};
