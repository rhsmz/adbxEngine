pub mod drag_handler;
pub mod resource;
pub mod ui_draw;

// 公開API
pub use drag_handler::{
    apply_layout_to_editor, detect_drop_zone, handle_panel_drag, load_layout, save_layout,
    toggle_panel_docking, toggle_panel_visibility, update_panel_layout, DockZone,
    DropZoneIndicator,
};
pub use resource::{
    initialize_docking_system, start_panel_drag, DockingSystem, PanelHeader, PanelState,
};
pub use ui_draw::{detect_resize_edge, draw_docking_ui, show_drop_zone, update_panel_drag};
