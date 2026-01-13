pub mod keyboard_input;
pub mod resource;

// 公開API
pub use keyboard_input::{
    copy_selection_code, copy_selection_script, cut_selection_code, cut_selection_script,
    handle_text_editor_input, paste_text_code, paste_text_script,
};
pub use resource::{EditorType, TextEditorState};
