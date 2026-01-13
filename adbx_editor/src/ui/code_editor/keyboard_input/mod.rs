pub mod character_input;
pub mod editing_operations;
pub mod navigation;
pub mod shortcuts;
pub mod completion_navigation;
mod main;

// 公開API
pub use main::handle_code_editor_keyboard_input;
pub use editing_operations::{undo_edit as undo_code_editor_edit, redo_edit as redo_code_editor_edit};
