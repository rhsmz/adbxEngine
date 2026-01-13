pub mod resource;
pub mod state;
pub mod render;
pub mod syntax_highlight;
pub mod completion;
pub mod keyboard_input;
pub mod cursor_position;
pub mod scroll;
pub mod file_operations;
pub mod ai_integration;

// 公開API
pub use resource::CodeEditor;
pub use state::{CompletionState, CompletionCandidate, CompletionKind, OpenFile};
pub use ai_integration::{AiIntegration, AiRequest, AiResponse, AiRequestType, AiProvider, AiRequestStatus};
pub use render::{draw_code_editor_ui, handle_code_editor_click};
pub use syntax_highlight::{apply_syntax_highlighting_to_code, detect_programming_language_from_file_path};
pub use completion::{trigger_code_completion_for_current_position, accept_selected_completion_candidate};
pub use keyboard_input::{handle_code_editor_keyboard_input, undo_code_editor_edit, redo_code_editor_edit};
pub use cursor_position::{calculate_cursor_line_and_column, calculate_char_position_from_line_column, calculate_cursor_x_pixel_position, calculate_cursor_position_from_mouse_click, get_line_length};
pub use scroll::handle_code_editor_mouse_wheel_scroll;
pub use file_operations::{open_file_in_code_editor, save_active_file_in_code_editor};
pub use ai_integration::{request_ai_code_generation, request_ai_code_completion, request_ai_code_refactor, process_pending_ai_code_generation_requests, handle_ai_code_generation_responses};
