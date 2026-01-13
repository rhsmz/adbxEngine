use adbx_runtime::AdbxRuntimePlugin;
use bevy::prelude::*;

/// アプリケーションの初期化
pub fn initialize_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Adbx Engine Editor".into(),
            resolution: bevy::window::WindowResolution::new(1920, 1080),
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(AdbxRuntimePlugin);
    app.add_plugins(super::plugin::EditorPlugin);
    app.init_resource::<crate::editor_app::EditorApp>();
    app.init_resource::<crate::systems::selection::Selection>();
    app.init_resource::<crate::systems::scene_management::SceneManager>();
    app.init_resource::<crate::systems::scene_management::LoadSceneRequest>();
    app.init_resource::<crate::systems::runtime_state::RuntimeStateManager>();
    app.init_resource::<crate::ui::scene_view::SceneView>();
    app.init_resource::<crate::ui::hierarchy::HierarchyView>();
    app.init_resource::<crate::ui::hierarchy::HierarchyDragState>();
    app.init_resource::<crate::ui::inspector::InspectorPanel>();
    app.init_resource::<crate::ui::inspector::InspectorInputState>();
    app.init_resource::<crate::ui::inspector::ComponentEditorRegistry>();
    app.insert_resource(crate::ui::asset_browser::AssetBrowser::new());
    app.insert_resource(crate::ui::log_panel::LogPanel::new());
    app.init_resource::<crate::ui::code_editor::CodeEditor>();
    app.init_resource::<crate::ui::code_editor::AiIntegration>();
    app.init_resource::<crate::ui::script_editor::ScriptEditor>();
    app.init_resource::<crate::project::Project>();
    app.init_resource::<crate::systems::menu::ProjectRequest>();
    app.init_resource::<crate::communication::EditorRuntimeCommunication>();
    app.init_resource::<crate::ui::editor_layout::EditorLayout>();
    app.init_resource::<crate::systems::performance::PerformanceSettings>();
    app.init_resource::<crate::systems::performance::FrameCounter>();
    app.init_resource::<crate::settings::EditorSettings>();
    app.init_resource::<crate::settings::SettingsPanel>();
    app.init_resource::<crate::ui::docking::DockingSystem>();
    app.insert_resource(crate::systems::operation_recording::OperationRecorder::new(
        1000,
    ));
    app.init_resource::<crate::ui::realtime_preview::RealtimePreview>();
    app.init_resource::<crate::ui::text_editor::TextEditorState>();
    app.init_resource::<crate::ui::context_menu::ContextMenu>();
    app.init_resource::<crate::ui::file_dialog::FileDialogRequest>();
    app.init_resource::<crate::ui::rename_dialog::RenameDialogRequest>();
    app.init_resource::<crate::ui::search_replace::SearchReplace>();
    app.init_resource::<crate::ui::error_dialog::ErrorDialog>();
    app.init_resource::<crate::ui::clipboard::Clipboard>();
    app.init_resource::<crate::systems::build_game::BuildGameRequest>();
    app.init_resource::<crate::systems::build_game::BuildProgress>();
    app
}
