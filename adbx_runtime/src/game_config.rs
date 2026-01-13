use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ゲーム設定リソース
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub scene_path: Option<PathBuf>,
    pub project_path: Option<PathBuf>,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            scene_path: None,
            project_path: None,
            window_title: "Adbx Game".to_string(),
            window_width: 1920,
            window_height: 1080,
        }
    }
}

/// ビルド設定ファイルの構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub game_name: String,
    pub main_scene: String,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub assets: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            game_name: "MyGame".to_string(),
            main_scene: "MainScene".to_string(),
            window_title: "My Game".to_string(),
            window_width: 1920,
            window_height: 1080,
            assets: vec![],
        }
    }
}

/// ビルド設定ファイルを読み込む
pub fn load_build_config(project_path: &PathBuf) -> Result<BuildConfig, String> {
    let config_path = project_path.join("build_config.json");

    if !config_path.exists() {
        // デフォルト設定を返す
        return Ok(BuildConfig::default());
    }

    let config_content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read build config: {}", e))?;

    let config: BuildConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse build config: {}", e))?;

    Ok(config)
}

/// ビルド設定ファイルを保存する
pub fn save_build_config(project_path: &PathBuf, config: &BuildConfig) -> Result<(), String> {
    let config_path = project_path.join("build_config.json");

    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize build config: {}", e))?;

    std::fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write build config: {}", e))?;

    Ok(())
}
