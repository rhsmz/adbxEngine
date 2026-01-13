pub mod menu_shortcuts;
pub mod menu_click_handler;
pub mod build_game_menu;

pub use menu_shortcuts::{handle_menu_shortcuts, handle_menu_shortcuts_settings};
pub use menu_click_handler::{handle_menu_click, ProjectRequest};
pub use build_game_menu::{handle_build_game_request, BuildGameMenuRequest};
