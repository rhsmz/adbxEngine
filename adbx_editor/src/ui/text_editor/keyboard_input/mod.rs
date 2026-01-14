pub mod code_editor_handler;
mod main;
pub mod navigation;
pub mod script_editor_handler;

// 公開API
pub use main::handle_text_editor_input;
