pub mod ai_integration;
pub mod completion;
pub mod cursor_position;
pub mod file_operations;
pub mod keyboard_input;
pub mod render;
pub mod resource;
pub mod scroll;
pub mod state;
pub mod syntax_highlight;

// 公開API
pub use ai_integration::{
    handle_ai_code_generation_responses, process_pending_ai_code_generation_requests,
};
pub use ai_integration::AiIntegration;
pub use file_operations::{open_file_in_code_editor, save_active_file_in_code_editor};
pub use keyboard_input::{
    handle_code_editor_keyboard_input, redo_code_editor_edit, undo_code_editor_edit,
};
pub use render::{draw_code_editor_ui, handle_code_editor_click};
pub use resource::CodeEditor;
pub use scroll::handle_code_editor_mouse_wheel_scroll;
pub use state::{CompletionCandidate, CompletionKind, CompletionState, OpenFile};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use ai_integration::{AiRequest, AiRequestType, AiResponse, request_ai_code_completion, request_ai_code_generation, request_ai_code_refactor};
#[allow(unused_imports)]
pub use ai_integration::{AiProvider, AiRequestStatus};
#[allow(unused_imports)]
pub use completion::{accept_selected_completion_candidate, trigger_code_completion_for_current_position};
#[allow(unused_imports)]
pub use cursor_position::{
    calculate_char_position_from_line_column, calculate_cursor_line_and_column,
    calculate_cursor_position_from_mouse_click, calculate_cursor_x_pixel_position, get_line_length,
};
#[allow(unused_imports)]
pub use syntax_highlight::{
    apply_syntax_highlighting_to_code, detect_programming_language_from_file_path,
};
