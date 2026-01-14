pub mod display;
pub mod input_handler;
pub mod resource;
pub mod show_functions;

// 公開API
pub use resource::{ErrorDialog, ErrorType};
pub use show_functions::{
    show_error_dialog, show_error_dialog_from_error, show_info_dialog, show_warning_dialog,
};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use display::draw_error_dialog;
#[allow(unused_imports)]
pub use input_handler::handle_error_dialog_input;
#[allow(unused_imports)]
pub use show_functions::close_error_dialog;
