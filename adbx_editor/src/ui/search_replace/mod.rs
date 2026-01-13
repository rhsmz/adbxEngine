pub mod resource;
pub mod dialog_display;
pub mod search_operation;
pub mod replace_operation;

// 公開API
pub use resource::{SearchReplace, SearchResult, show_search_dialog, show_replace_dialog};
pub use dialog_display::draw_search_replace;
pub use search_operation::{perform_search, handle_search_replace_input};
pub use replace_operation::{perform_replace, perform_replace_all, validate_replace, display_replace_result};
