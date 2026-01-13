pub mod dialog_display;
pub mod replace_operation;
pub mod resource;
pub mod search_operation;

// 公開API
pub use dialog_display::draw_search_replace;
pub use replace_operation::{
    display_replace_result, perform_replace, perform_replace_all, validate_replace,
};
pub use resource::{show_replace_dialog, show_search_dialog, SearchReplace, SearchResult};
pub use search_operation::{handle_search_replace_input, perform_search};
