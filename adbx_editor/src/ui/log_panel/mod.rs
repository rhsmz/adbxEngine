pub mod resource;
pub mod log_display;

pub use resource::{LogPanel, LogEntry, LogLevel};
pub use log_display::{draw_log_panel, toggle_log_panel};
