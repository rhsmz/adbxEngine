use super::super::editor_settings::EditorSettings;
use super::super::project_settings::ProjectSettings;

/// 設定の検証
pub fn validate_editor_settings(settings: &EditorSettings) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // フォントサイズの検証
    if settings.font_size < 8.0 || settings.font_size > 24.0 {
        errors.push("Font size must be between 8.0 and 24.0".to_string());
    }

    // 自動保存間隔の検証
    if settings.auto_save_interval < 10 {
        errors.push("Auto save interval must be at least 10 seconds".to_string());
    }

    // AIプロバイダーの検証
    if settings.ai_provider.is_empty() {
        errors.push("AI provider cannot be empty".to_string());
    }

    // タブ幅の検証
    if settings.tab_width == 0 || settings.tab_width > 8 {
        errors.push("Tab width must be between 1 and 8".to_string());
    }

    // Undo履歴の検証
    if settings.max_undo_history == 0 || settings.max_undo_history > 10000 {
        errors.push("Max undo history must be between 1 and 10000".to_string());
    }

    // エディタ言語の検証
    if settings.editor_language.is_empty() {
        errors.push("Editor language cannot be empty".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// プロジェクト設定の検証
pub fn validate_project_settings(settings: &ProjectSettings) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // プロジェクト名の検証
    if settings.name.is_empty() {
        errors.push("Project name cannot be empty".to_string());
    }

    // バージョンの検証（セマンティックバージョニング形式）
    if !settings.version.is_empty() {
        let version_parts: Vec<&str> = settings.version.split('.').collect();
        if version_parts.len() != 3 {
            errors.push("Version must be in semantic versioning format (e.g., 1.0.0)".to_string());
        } else {
            for part in &version_parts {
                if part.parse::<u32>().is_err() {
                    errors.push("Version parts must be numeric".to_string());
                    break;
                }
            }
        }
    }

    // ターゲットFPSの検証
    if let Some(fps) = settings.target_fps {
        if fps == 0 || fps > 1000 {
            errors.push("Target FPS must be between 1 and 1000".to_string());
        }
    }

    // オーディオサンプルレートの検証
    if settings.audio_sample_rate < 8000 || settings.audio_sample_rate > 192000 {
        errors.push("Audio sample rate must be between 8000 and 192000".to_string());
    }

    // アセットパスの検証
    if settings.asset_paths.is_empty() {
        errors.push("At least one asset path must be specified".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
