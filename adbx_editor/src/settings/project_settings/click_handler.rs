use super::super::editor_settings::resource::{EditorSettings, SettingsPanel};
use bevy::prelude::*;

/// 設定パネルのクリック処理（プロジェクト設定関連）
#[allow(dead_code)]
pub fn handle_project_settings_panel_click(
    mut settings_panel: ResMut<SettingsPanel>,
    _editor_settings: ResMut<EditorSettings>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, name) in interaction_query.iter() {
        if *interaction == bevy::ui::Interaction::Pressed {
            let name_str = name.as_str();

            // 閉じるボタン
            if name_str == "SettingsPanelCloseButton" {
                settings_panel.is_open = false;
                if let Some(content_entity) = settings_panel.content_entity {
                    if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                        entity_commands.despawn();
                    }
                    settings_panel.content_entity = None;
                }
                return;
            }

            // 保存ボタン
            if name_str == "SettingsPanelSaveButton" {
                // 設定を保存する処理（実装は後で追加）
                bevy::log::info!("Settings saved");
                return;
            }

            // キャンセルボタン
            if name_str == "SettingsPanelCancelButton" {
                settings_panel.is_open = false;
                if let Some(content_entity) = settings_panel.content_entity {
                    if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                        entity_commands.despawn();
                    }
                    settings_panel.content_entity = None;
                }
                return;
            }
        }
    }
}
