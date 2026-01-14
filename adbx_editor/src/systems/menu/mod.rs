pub mod build_game_menu;
pub mod menu_click_handler;
pub mod menu_shortcuts;

pub use build_game_menu::handle_build_game_request;
pub use menu_click_handler::{handle_menu_click, ProjectRequest};
pub use menu_shortcuts::{handle_menu_shortcuts, handle_menu_shortcuts_settings};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use build_game_menu::BuildGameMenuRequest;
