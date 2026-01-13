use super::ScriptEditor;
use bevy::prelude::*;
use bevy::ui::Interaction;

/// スクリプトエディタのクリック処理
pub fn handle_script_editor_click(
    mut script_editor: ResMut<ScriptEditor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                let name_str = name.as_str();

                // エディタエリアのクリック（フォーカス）
                if name_str == "ScriptEditorArea" {
                    script_editor.is_focused = true;
                    // クリック位置からカーソル位置を計算（簡易実装）
                    if let Some(window) = windows.iter().next() {
                        if let Some(_cursor_pos) = window.cursor_position() {
                            // TODO: より正確なカーソル位置計算
                            script_editor.cursor_position = script_editor
                                .cursor_position
                                .min(script_editor.content.len());
                        }
                    }
                    script_editor.content_entity = None;
                }

                // 保存ボタンがクリックされた場合
                if name_str == "ScriptEditorSaveButton" {
                    if let Some(script_path) = &script_editor.current_script {
                        if let Err(e) = std::fs::write(script_path, &script_editor.content) {
                            bevy::log::error!("Failed to save script: {}", e);
                        } else {
                            bevy::log::info!("Script saved: {:?}", script_path);
                        }
                    } else {
                        bevy::log::warn!("No script file selected. Cannot save.");
                    }
                }
            }
        }
    }
}
