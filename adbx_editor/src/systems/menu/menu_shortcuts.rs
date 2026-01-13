use bevy::prelude::*;
use crate::project::Project;
use crate::systems::scene_management::{save_current_scene, SceneManager, list_available_scenes, LoadSceneRequest};

/// メニューのキーボードショートカット処理
pub fn handle_menu_shortcuts(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    project: Res<Project>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
    mut load_scene_request: ResMut<LoadSceneRequest>,
    entities: Query<(Entity, &Name), With<Transform>>,
    children_query: Query<&Children>,
    transform_query: Query<&Transform>,
    name_query: Query<&Name>,
    mut scene_view: ResMut<crate::ui::scene_view::SceneView>,
    mut operation_recorder: Option<ResMut<crate::systems::operation_recording::OperationRecorder>>,
) {
    // Ctrl+S: シーンを保存
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            save_current_scene(
                project.as_ref(),
                &mut scene_manager,
                operation_recorder.as_mut().map(|r| r as &mut ResMut<_>),
                &mut commands,
                &entities,
                &children_query,
                &transform_query,
                &name_query,
            );
        }
        // Ctrl+O: シーンを読み込み（最初の利用可能なシーンを読み込む簡易実装）
        if keyboard_input.just_pressed(KeyCode::KeyO) {
            let available_scenes = list_available_scenes(project.as_ref());
            if let Some(first_scene) = available_scenes.first() {
                load_scene_request.scene_name = Some(first_scene.clone());
            } else {
                bevy::log::warn!("No scenes available to load.");
            }
        }
        // Ctrl+B: ゲームをビルド
        if keyboard_input.just_pressed(KeyCode::KeyB) {
            bevy::log::info!("Build game shortcut pressed");
        }
    }
    
    // Ctrl+,: 設定パネルを開く（handle_menu_shortcuts_settingsで処理）
    
    // Gizmoモードの切り替え
    // T: Translate (移動)
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        scene_view.gizmo_mode = crate::ui::scene_view::GizmoMode::Translate;
    }
    // R: Rotate (回転)
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        scene_view.gizmo_mode = crate::ui::scene_view::GizmoMode::Rotate;
    }
    // S: Scale (スケール) - Ctrl+Sと競合しないように、Ctrlが押されていない場合のみ
    if !keyboard_input.pressed(KeyCode::ControlLeft) && !keyboard_input.pressed(KeyCode::ControlRight) {
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            scene_view.gizmo_mode = crate::ui::scene_view::GizmoMode::Scale;
        }
    }
    // X: モード解除
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        scene_view.gizmo_mode = crate::ui::scene_view::GizmoMode::None;
    }
}

/// 設定パネルを開くショートカット処理
pub fn handle_menu_shortcuts_settings(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings_panel: ResMut<crate::settings::SettingsPanel>,
) {
    // Ctrl+,: 設定パネルを開く
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
        if keyboard_input.just_pressed(KeyCode::Comma) {
            settings_panel.is_open = !settings_panel.is_open;
        }
    }
}
