use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Editing,
    Playing,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<EditorState>()
            .add_systems(Startup, (
                super::setup::setup_editor,
                crate::ui::scene_view::setup_scene_view,
                crate::ui::build_editor_ui,
            ))
            .add_systems(Update, crate::systems::performance::update_frame_counter)
            .add_systems(Update, (
                super::ui_drawing::draw_editor_ui,
                crate::systems::selection::handle_selection,
                crate::systems::gizmo::draw_gizmos,
                crate::systems::gizmo::handle_gizmo_interaction.ambiguous_with(crate::systems::selection::handle_selection),
                crate::systems::menu::handle_menu_shortcuts,
                crate::systems::menu::handle_menu_shortcuts_settings,
                crate::systems::menu::handle_menu_click,
                crate::systems::menu::handle_build_game_request,
                crate::systems::scene_management::handle_load_scene_request,
            ))
            .add_systems(Update, (
                crate::ui::hierarchy::draw_hierarchy_view_ui,
                crate::ui::hierarchy::handle_hierarchy_view_item_click,
                crate::ui::hierarchy::handle_hierarchy_view_drag_and_drop,
                crate::ui::hierarchy::handle_hierarchy_entity_create_and_delete,
                crate::ui::inspector::draw_inspector_panel_ui,
                crate::ui::inspector::handle_inspector_transform_input,
            ))
            .add_systems(Update, (
                crate::ui::scene_view::handle_scene_view_input,
                crate::ui::draw_asset_browser,
                crate::ui::handle_asset_browser_click,
                crate::ui::draw_script_editor,
                crate::ui::handle_script_editor_click,
                crate::ui::log_panel::draw_log_panel,
                crate::communication::handle_runtime_messages,
            ))
            .add_systems(Update, (
                crate::systems::realtime_sync::sync_entity_selection,
                crate::systems::realtime_sync::sync_transform_changes,
                crate::systems::realtime_sync::apply_runtime_entity_updates,
            ))
            .add_systems(Update, (
                crate::ui::search_replace::draw_search_replace,
                crate::ui::search_replace::handle_search_replace_input,
            ))
            .add_systems(Update, (
                crate::ui::rename_dialog::draw_rename_dialog,
                crate::ui::rename_dialog::handle_rename_dialog_click,
            ))
            .add_systems(Update, crate::ui::editor_layout::handle_toolbar_click)
            .add_systems(Update, crate::ui::context_menu::detect_right_click_for_context_menu)
            .add_systems(Update, crate::ui::context_menu::handle_context_menu_item_click)
            .add_systems(Update, crate::ui::context_menu::handle_script_execution)
            .add_systems(Update, (
                crate::ui::code_editor::draw_code_editor_ui,
                crate::ui::code_editor::handle_code_editor_click,
                crate::ui::code_editor::handle_code_editor_mouse_wheel_scroll,
                crate::ui::code_editor::handle_code_editor_keyboard_input,
                crate::ui::code_editor::process_pending_ai_code_generation_requests,
                crate::ui::code_editor::handle_ai_code_generation_responses,
                crate::ui::script_editor::handle_script_editor_keyboard_input,
                crate::ui::asset_browser::update_image_previews,
            ))
            .add_systems(Update, (
                crate::settings::draw_settings_panel,
                crate::settings::handle_settings_panel_click,
                crate::settings::apply_settings_changes,
                crate::settings::save_editor_settings_system,
            ))
            .add_systems(Update, (
                crate::ui::realtime_preview::update_preview_on_code_change,
                crate::ui::realtime_preview::draw_realtime_preview,
                crate::ui::realtime_preview::handle_preview_window_click,
            ))
            .add_systems(Startup, crate::ui::inspector::register_default_component_editors)
            .add_systems(Update, crate::ui::script_editor::validate_script_system)
            .init_resource::<crate::systems::gizmo::GizmoInteraction>();
    }
}
