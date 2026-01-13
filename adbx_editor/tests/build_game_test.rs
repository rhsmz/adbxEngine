#[cfg(test)]
mod tests {
    use adbx_editor::systems::build_game::*;

    #[test]
    fn test_build_game_request_default() {
        let request = BuildGameRequest::default();
        assert!(!request.is_building);
        assert!(request.build_output.is_empty());
        assert!(request.build_error.is_none());
    }

    #[test]
    fn test_build_progress_default() {
        let progress = BuildProgress::default();
        assert!(!progress.is_visible);
        assert!(progress.progress_text.is_empty());
        assert_eq!(progress.progress_percent, 0.0);
    }

    #[test]
    fn test_build_config_structure() {
        let config = BuildConfig {
            game_name: "Test Game".to_string(),
            main_scene: "MainScene".to_string(),
            window_title: "Test Game".to_string(),
            window_width: 1920,
            window_height: 1080,
            assets: vec!["assets".to_string()],
        };
        assert_eq!(config.game_name, "Test Game");
        assert_eq!(config.main_scene, "MainScene");
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert_eq!(config.assets.len(), 1);
    }
}
