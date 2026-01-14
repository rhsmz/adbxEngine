pub mod highlight_application;
pub mod language_detection;
pub mod style_application;

pub use highlight_application::apply_syntax_highlighting_to_code;
pub use language_detection::detect_programming_language_from_file_path;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use highlight_application::highlight_code_text_by_line;
#[allow(unused_imports)]
pub use language_detection::get_extension_from_language;
#[allow(unused_imports)]
pub use style_application::{convert_syntect_style_to_bevy_color, get_default_text_color};
