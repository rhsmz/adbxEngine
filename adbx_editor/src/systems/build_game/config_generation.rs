use crate::project::Project;
use std::path::PathBuf;

/// ビルド設定ファイルの構造
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildConfig {
    pub game_name: String,
    pub main_scene: String,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub assets: Vec<String>,
}

/// ビルド設定ファイルを生成
pub fn generate_build_config(project_path: &PathBuf, project: &Project) -> Result<(), String> {
    // 利用可能なシーンを取得
    let scenes = crate::project::list_scenes(project_path)?;
    let main_scene = scenes
        .first()
        .cloned()
        .unwrap_or_else(|| "MainScene".to_string());

    // アセットディレクトリを取得
    let assets_dir = project_path.join("assets");
    let mut assets = Vec::new();

    if assets_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&assets_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(path_str) = path.to_str() {
                        assets.push(path_str.to_string());
                    }
                }
            }
        }
    }

    let build_config = BuildConfig {
        game_name: project.name.clone(),
        main_scene,
        window_title: format!("{}", project.name),
        window_width: 1920,
        window_height: 1080,
        assets,
    };

    let config_path = project_path.join("build_config.json");
    let config_json = serde_json::to_string_pretty(&build_config)
        .map_err(|e| format!("Failed to serialize build config: {}", e))?;

    std::fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write build config: {}", e))?;

    Ok(())
}
