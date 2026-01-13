pub mod app_initialization;
pub mod system_registration;
pub mod plugin;
pub mod setup;
pub mod ui_drawing;
pub mod layout_loading;

pub use plugin::{EditorPlugin, EditorState};
pub use setup::setup_editor;
pub use ui_drawing::draw_editor_ui;
pub use layout_loading::load_docking_layout;
