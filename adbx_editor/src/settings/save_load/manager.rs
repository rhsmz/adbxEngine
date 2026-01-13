use std::path::PathBuf;
use super::super::editor_settings::EditorSettings;
use super::super::project_settings::ProjectSettings;

/// 設定管理システム
pub struct SettingsManager;

impl SettingsManager {
    /// エディタ設定を保存
    pub fn save_editor_settings(settings: &EditorSettings) -> Result<(), String> {
        let config_dir = Self::get_config_dir()?;
        let settings_path = config_dir.join("editor_settings.json");
        
        let settings_json = serde_json::to_string_pretty(settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        
        std::fs::write(&settings_path, settings_json)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;
        
        Ok(())
    }
    
    /// エディタ設定を読み込み
    pub fn load_editor_settings() -> Result<EditorSettings, String> {
        let config_dir = Self::get_config_dir()?;
        let settings_path = config_dir.join("editor_settings.json");
        
        if !settings_path.exists() {
            return Ok(EditorSettings::default());
        }
        
        let settings_content = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;
        
        let settings: EditorSettings = serde_json::from_str(&settings_content)
            .map_err(|e| format!("Failed to parse settings: {}", e))?;
        
        Ok(settings)
    }
    
    /// プロジェクト設定を保存
    pub fn save_project_settings(
        settings: &ProjectSettings,
        project_path: &PathBuf,
    ) -> Result<(), String> {
        let settings_path = project_path.join("project_settings.json");
        
        let settings_json = serde_json::to_string_pretty(settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        
        std::fs::write(&settings_path, settings_json)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;
        
        Ok(())
    }
    
    /// プロジェクト設定を読み込み
    pub fn load_project_settings(project_path: &PathBuf) -> Result<ProjectSettings, String> {
        let settings_path = project_path.join("project_settings.json");
        
        if !settings_path.exists() {
            return Ok(ProjectSettings::default());
        }
        
        let settings_content = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;
        
        let settings: ProjectSettings = serde_json::from_str(&settings_content)
            .map_err(|e| format!("Failed to parse settings: {}", e))?;
        
        Ok(settings)
    }
    
    /// 設定ディレクトリを取得
    pub fn get_config_dir() -> Result<PathBuf, String> {
        let config_dir = dirs::config_dir()
            .ok_or("Failed to get config directory")?
            .join("adbx_engine");
        
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
        
        Ok(config_dir)
    }
}
