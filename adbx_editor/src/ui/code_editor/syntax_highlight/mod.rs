pub mod highlight_application;
pub mod language_detection;
pub mod style_application;

pub use highlight_application::{apply_syntax_highlighting_to_code, highlight_code_text_by_line};
pub use language_detection::{
    detect_programming_language_from_file_path, get_extension_from_language,
};
pub use style_application::{convert_syntect_style_to_bevy_color, get_default_text_color};
