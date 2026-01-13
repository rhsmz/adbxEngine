use super::ScriptEditor;
use bevy::prelude::*;
use std::path::PathBuf;

/// スクリプトファイルを読み込む
pub fn load_script_file(script_path: PathBuf, script_editor: &mut ScriptEditor) {
    match std::fs::read_to_string(&script_path) {
        Ok(content) => {
            script_editor.current_script = Some(script_path.clone());
            script_editor.content = content;
            script_editor.cursor_position = 0;
            script_editor.selection_start = None;
            // コンテンツエンティティをクリアして再描画を促す
            script_editor.content_entity = None;
            bevy::log::info!("Script loaded: {:?}", script_path);
        }
        Err(e) => {
            bevy::log::error!("Failed to load script: {}", e);
        }
    }
}
