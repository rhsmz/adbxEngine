pub mod click_detection;
pub mod entity_operations;
pub mod macros;
pub mod menu_display;
pub mod menu_items;
pub mod types;

// 公開API
pub use click_detection::detect_right_click_for_context_menu;
pub use entity_operations::handle_entity_operations;
pub use menu_display::{hide_context_menu, show_context_menu};
pub use types::{
    AssetOperation, ContextMenu, ContextType, EntityClipboardOperation, EntityOperation,
};
