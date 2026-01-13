pub mod drag_start;
pub mod drag_update;
pub mod drop_processing;
pub mod panel_management;
pub mod layout_operations;

pub use drag_start::handle_panel_drag;
pub use drag_update::update_panel_layout;
pub use drop_processing::{detect_drop_zone, DockZone, DropZoneIndicator};
pub use panel_management::{toggle_panel_visibility, toggle_panel_docking};
pub use layout_operations::{save_layout, load_layout, apply_layout_to_editor};
