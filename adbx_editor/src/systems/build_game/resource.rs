use bevy::prelude::*;

/// ビルドゲームリクエストリソース
#[derive(Resource, Default)]
pub struct BuildGameRequest {
    pub is_building: bool,
    pub build_output: Vec<String>,
    pub build_error: Option<String>,
}

/// ビルド進捗リソース
#[derive(Resource, Default)]
pub struct BuildProgress {
    pub is_visible: bool,
    pub progress_text: String,
    pub progress_percent: f32,
}
