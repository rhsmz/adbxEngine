use super::super::project_settings::ProjectSettings;
use super::manager::SettingsManager;
use std::path::PathBuf;

/// プロジェクト設定を保存
pub fn save_project_settings(
    settings: &ProjectSettings,
    project_path: &PathBuf,
) -> Result<(), String> {
    SettingsManager::save_project_settings(settings, project_path)
}

/// プロジェクト設定を読み込み
pub fn load_project_settings(project_path: &PathBuf) -> Result<ProjectSettings, String> {
    SettingsManager::load_project_settings(project_path)
}
