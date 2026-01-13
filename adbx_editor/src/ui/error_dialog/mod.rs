pub mod resource;
pub mod show_functions;
pub mod display;
pub mod input_handler;

// 公開API
pub use resource::{ErrorDialog, ErrorType};
pub use show_functions::{
    show_error_dialog,
    show_error_dialog_from_error,
    show_warning_dialog,
    show_info_dialog,
    close_error_dialog,
};
pub use display::draw_error_dialog;
pub use input_handler::handle_error_dialog_input;
