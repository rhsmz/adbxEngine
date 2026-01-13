pub mod resource;
pub mod ui_draw;
pub mod drag_handler;

// 公開API
pub use resource::{DockingSystem, PanelState, PanelHeader, initialize_docking_system, start_panel_drag};
pub use ui_draw::{draw_docking_ui, update_panel_drag, detect_resize_edge, show_drop_zone};
pub use drag_handler::{handle_panel_drag, update_panel_layout, detect_drop_zone, DockZone, DropZoneIndicator, toggle_panel_visibility, toggle_panel_docking, save_layout, load_layout, apply_layout_to_editor};
