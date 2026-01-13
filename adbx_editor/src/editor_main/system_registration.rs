use bevy::prelude::*;

/// システムの登録
pub fn register_systems(app: &mut App) {
    app
        .add_systems(Startup, (
            crate::settings::load_editor_settings_on_startup,
            crate::ui::docking::initialize_docking_system,
            super::layout_loading::load_docking_layout,
        ))
        .add_systems(Update, (
            crate::ui::docking::apply_layout_to_editor,
            crate::ui::docking::update_panel_layout,
        ))
        .add_systems(Update, (
            crate::ui::docking::start_panel_drag,
            crate::ui::docking::update_panel_drag,
            crate::ui::text_editor::handle_text_editor_input,
        ));
}
