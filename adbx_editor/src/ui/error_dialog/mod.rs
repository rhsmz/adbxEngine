pub mod display;
pub mod input_handler;
pub mod resource;
pub mod show_functions;

// 公開API
pub use display::draw_error_dialog;
pub use input_handler::handle_error_dialog_input;
pub use resource::{ErrorDialog, ErrorType};
pub use show_functions::{
    close_error_dialog, show_error_dialog, show_error_dialog_from_error, show_info_dialog,
    show_warning_dialog,
};
