pub mod resource;
pub mod preview_generation;
pub mod ui_draw;
pub mod click_handler;
pub mod update_handler;

// 公開API
pub use resource::{RealtimePreview, PreviewContent, PreviewType};
pub use preview_generation::{update_preview, detect_preview_type};
pub use ui_draw::draw_realtime_preview;
pub use click_handler::handle_preview_window_click;
pub use update_handler::update_preview_on_code_change;
