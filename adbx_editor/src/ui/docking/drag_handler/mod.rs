pub mod drag_start;
pub mod drag_update;
pub mod drop_processing;
pub mod layout_operations;
pub mod panel_management;

pub use drag_start::handle_panel_drag;
pub use drag_update::update_panel_layout;
pub use drop_processing::{detect_drop_zone, DockZone, DropZoneIndicator};
pub use layout_operations::{apply_layout_to_editor, load_layout, save_layout};
pub use panel_management::{toggle_panel_docking, toggle_panel_visibility};
