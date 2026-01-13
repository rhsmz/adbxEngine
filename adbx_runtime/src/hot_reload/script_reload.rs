use crate::lua::component::{LuaScript, LuaScriptState};
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// スクリプトファイルの変更を監視するリソース
#[derive(Resource, Default)]
pub struct ScriptWatcher {
    pub watched_scripts: HashMap<PathBuf, std::time::SystemTime>,
}

/// スクリプトファイルの変更を検出して再読み込みするシステム
pub fn watch_and_reload_scripts(
    mut script_watcher: ResMut<ScriptWatcher>,
    mut query: Query<(Entity, &mut LuaScript, &mut LuaScriptState), With<LuaScript>>,
) {
    for (entity, mut script, mut state) in query.iter_mut() {
        // ファイルの最終更新時刻をチェック
        if let Ok(metadata) = std::fs::metadata(&script.script_path) {
            if let Ok(modified) = metadata.modified() {
                let should_reload = script_watcher
                    .watched_scripts
                    .get(&script.script_path)
                    .map(|&prev| prev < modified)
                    .unwrap_or(true);

                if should_reload {
                    // ファイルを再読み込み
                    if let Ok(content) = std::fs::read_to_string(&script.script_path) {
                        script.script_content = content;
                        *state = LuaScriptState::Loaded;
                        script_watcher
                            .watched_scripts
                            .insert(script.script_path.clone(), modified);
                        bevy::log::info!(
                            "Reloaded script: {:?} for entity {:?}",
                            script.script_path,
                            entity
                        );
                    }
                } else {
                    // 最終更新時刻を記録
                    script_watcher
                        .watched_scripts
                        .insert(script.script_path.clone(), modified);
                }
            }
        }
    }
}
