use super::super::editor_settings::EditorSettings;
use super::manager::SettingsManager;

/// エディタ設定を保存
#[allow(dead_code)]
pub fn save_editor_settings(settings: &EditorSettings) -> Result<(), String> {
    SettingsManager::save_editor_settings(settings)
}

/// エディタ設定を読み込み
#[allow(dead_code)]
pub fn load_editor_settings() -> Result<EditorSettings, String> {
    SettingsManager::load_editor_settings()
}
