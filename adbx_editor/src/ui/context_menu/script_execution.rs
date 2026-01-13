use bevy::prelude::*;
use crate::communication::{EditorRuntimeCommunication, send_to_runtime};
use crate::ui::script_editor::ScriptEditor;

/// スクリプト実行処理
pub fn handle_script_execution(
    mut communication: ResMut<EditorRuntimeCommunication>,
    mut script_editor: ResMut<ScriptEditor>,
) {
    if script_editor.should_execute {
        script_editor.should_execute = false;
        // スクリプトを実行
        let script_content = script_editor.content.clone();
        if script_content.is_empty() {
            bevy::log::warn!("Script is empty. Cannot execute.");
        } else {
            // ランタイムにスクリプト実行を送信
            if let Err(e) = send_to_runtime(
                &mut communication,
                adbx_shared::EditorMessage::ExecuteScript {
                    script_content,
                },
            ) {
                bevy::log::error!("Failed to send script execution request: {}", e);
            } else {
                bevy::log::info!("Script execution request sent to runtime");
            }
        }
    }
}

