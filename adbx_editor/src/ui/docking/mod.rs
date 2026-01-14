pub mod drag_handler;
pub mod resource;
pub mod ui_draw;

// 公開API
pub use drag_handler::{apply_layout_to_editor, load_layout, update_panel_layout};
pub use resource::{initialize_docking_system, start_panel_drag, DockingSystem, PanelHeader};
pub use ui_draw::update_panel_drag;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use drag_handler::{detect_drop_zone, handle_panel_drag, save_layout, toggle_panel_docking, toggle_panel_visibility, DockZone, DropZoneIndicator};
#[allow(unused_imports)]
pub use resource::PanelState;
#[allow(unused_imports)]
pub use ui_draw::{detect_resize_edge, draw_docking_ui, show_drop_zone};
