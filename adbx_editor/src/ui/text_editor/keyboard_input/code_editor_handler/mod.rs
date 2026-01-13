pub mod character_input;
pub mod editing_operations;
pub mod navigation;
pub mod shortcuts;
mod main;

pub use main::handle_code_editor_keyboard_input;
pub use character_input::insert_text_at_cursor;
pub use editing_operations::{handle_backspace_code_editor, handle_delete_code_editor, undo_edit, redo_edit};
pub use navigation::{handle_arrow_left_code_editor, handle_arrow_right_code_editor, handle_arrow_up_code_editor, handle_arrow_down_code_editor, handle_home_code_editor, handle_end_code_editor};
pub use shortcuts::{copy_selection, paste_text, cut_selection, select_all};
