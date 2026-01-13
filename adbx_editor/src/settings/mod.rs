pub mod editor_settings;
pub mod project_settings;
pub mod save_load;

// 公開API
pub use editor_settings::{draw_settings_panel, EditorSettings, SettingsPanel};
pub use save_load::{
    apply_settings_changes, handle_settings_panel_click,
    load_editor_settings_on_startup,
    save_editor_settings_system,
};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use project_settings::{BuildSettings, ProjectSettings};
#[allow(unused_imports)]
pub use save_load::{
    load_editor_settings, load_project_settings, save_editor_settings, save_project_settings,
};
