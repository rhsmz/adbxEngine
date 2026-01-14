pub mod click_detection;
pub mod entity_operations;
pub mod macros;
pub mod menu_display;
pub mod menu_items;
pub mod types;

// 公開API
pub use menu_display::{hide_context_menu, show_context_menu};
pub use types::{
    AssetOperation, ContextMenu, ContextType, EntityClipboardOperation, EntityOperation,
};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use click_detection::detect_right_click_for_context_menu;
#[allow(unused_imports)]
pub use entity_operations::handle_entity_operations;
