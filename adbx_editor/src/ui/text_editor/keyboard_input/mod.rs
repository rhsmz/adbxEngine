pub mod navigation;
pub mod code_editor_handler;
pub mod script_editor_handler;
mod main;

// 公開API
pub use main::handle_text_editor_input;
pub use code_editor_handler::{copy_selection as copy_selection_code, paste_text as paste_text_code, cut_selection as cut_selection_code};
pub use script_editor_handler::{copy_selection as copy_selection_script, paste_text as paste_text_script, cut_selection as cut_selection_script};
