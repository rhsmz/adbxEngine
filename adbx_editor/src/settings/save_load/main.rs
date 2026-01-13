use super::super::editor_settings::{EditorSettings, SettingsPanel};
use super::manager::SettingsManager;
use super::validation::validate_editor_settings;
use bevy::prelude::*;

/// 設定パネルのクリック処理
pub fn handle_settings_panel_click(
    mut settings_panel: ResMut<SettingsPanel>,
    editor_settings: ResMut<EditorSettings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == bevy::ui::Interaction::Pressed {
                let name_str = name.as_str();

                // 閉じるボタンまたはキャンセルボタン
                if name_str == "SettingsPanelCloseButton" || name_str == "SettingsPanelCancelButton"
                {
                    settings_panel.is_open = false;
                    settings_panel.content_entity = None;
                }

                // 保存ボタン
                if name_str == "SettingsPanelSaveButton" {
                    // 設定を保存
                    if let Err(e) = SettingsManager::save_editor_settings(&editor_settings) {
                        bevy::log::error!("Failed to save editor settings: {}", e);
                    } else {
                        bevy::log::info!("Editor settings saved");
                        settings_panel.is_open = false;
                        settings_panel.content_entity = None;
                    }
                }
            }
        }
    }
}

/// 設定の適用システム（設定変更の即時反映）
pub fn apply_settings_changes(
    editor_settings: Res<EditorSettings>,
    mut windows: Query<&mut Window>,
) {
    if editor_settings.is_changed() {
        // ウィンドウサイズの適用
        if let Some(mut window) = windows.iter_mut().next() {
            let (width, height) = editor_settings.window_size;
            window.resolution.set(width as f32, height as f32);
        }

        // 設定の検証
        if let Err(errors) = validate_editor_settings(&editor_settings) {
            for error in errors {
                bevy::log::warn!("Settings validation warning: {}", error);
            }
        }
    }
}

/// 設定を更新するシステム（UIからの入力を受け取って更新）
pub fn update_settings_from_ui(
    _editor_settings: ResMut<EditorSettings>,
    _keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // 設定の更新処理
    // 実際の実装では、UIからの入力を受け取って更新
    // ここでは、キーボードショートカットによる設定変更の例
}

/// エディタ起動時に設定を読み込む
pub fn load_editor_settings_on_startup(mut editor_settings: ResMut<EditorSettings>) {
    match SettingsManager::load_editor_settings() {
        Ok(settings) => {
            *editor_settings = settings;
            bevy::log::info!("Editor settings loaded");
        }
        Err(e) => {
            bevy::log::warn!("Failed to load editor settings: {}. Using defaults.", e);
        }
    }
}

/// エディタ設定を保存するシステム（自動保存対応）
pub fn save_editor_settings_system(
    editor_settings: Res<EditorSettings>,
    mut save_timer: Local<Option<bevy::time::Timer>>,
    time: Res<Time>,
) {
    // 設定が変更された場合に自動保存
    if editor_settings.is_changed() {
        // 検証を実行
        if let Err(errors) = validate_editor_settings(&editor_settings) {
            for error in errors {
                bevy::log::error!("Settings validation failed: {}. Settings not saved.", error);
            }
            return;
        }

        // 自動保存が有効な場合、タイマーを設定
        if editor_settings.auto_save {
            if save_timer.is_none() {
                *save_timer = Some(bevy::time::Timer::from_seconds(
                    editor_settings.auto_save_interval as f32,
                    bevy::time::TimerMode::Once,
                ));
            }
        } else {
            // 自動保存が無効な場合、即座に保存
            if let Err(e) = SettingsManager::save_editor_settings(&editor_settings) {
                bevy::log::error!("Failed to save editor settings: {}", e);
            } else {
                bevy::log::info!("Editor settings saved");
            }
        }
    }

    // タイマーが設定されている場合、時間をチェック
    if let Some(ref mut timer) = *save_timer {
        timer.tick(time.delta());
        if timer.is_finished() {
            if let Err(e) = SettingsManager::save_editor_settings(&editor_settings) {
                bevy::log::error!("Failed to save editor settings: {}", e);
            } else {
                bevy::log::info!("Editor settings auto-saved");
            }
            *save_timer = None;
        }
    }
}
