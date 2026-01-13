pub mod resource;
pub mod keyboard_input;

// 公開API
pub use resource::{TextEditorState, EditorType};
pub use keyboard_input::{
    handle_text_editor_input,
    copy_selection_code,
    copy_selection_script,
    paste_text_code,
    paste_text_script,
    cut_selection_code,
    cut_selection_script,
};
