pub mod keyboard_input;
pub mod resource;

// 公開API
pub use keyboard_input::handle_text_editor_input;
pub use resource::TextEditorState;
