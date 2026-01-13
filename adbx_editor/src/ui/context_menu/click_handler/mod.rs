pub mod entity_operations;
pub mod clipboard_operations;
pub mod text_editor_operations;
pub mod search_replace_operations;
pub mod script_operations;
pub mod asset_operations;
pub mod scene_operations;
mod main;

// 公開API
pub use main::handle_context_menu_item_click;
pub use entity_operations::handle_entity_operation_menu_item;
pub use clipboard_operations::handle_clipboard_operation_menu_item;
pub use text_editor_operations::handle_text_editor_operation_menu_item;
pub use search_replace_operations::handle_search_replace_operation_menu_item;
pub use script_operations::handle_script_operation_menu_item;
pub use asset_operations::handle_asset_operation_menu_item;
pub use scene_operations::handle_scene_operation_menu_item;
