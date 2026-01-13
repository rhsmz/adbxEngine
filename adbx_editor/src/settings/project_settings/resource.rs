use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::project::SerializationFormat;

/// プロジェクト設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    pub version: String,
    pub target_fps: Option<u32>,
    pub audio_sample_rate: u32,
    pub asset_paths: Vec<PathBuf>,
    pub serialization_format: SerializationFormat,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            name: "Untitled Project".to_string(),
            version: "1.0.0".to_string(),
            target_fps: Some(60),
            audio_sample_rate: 44100,
            asset_paths: vec![PathBuf::from("assets")],
            serialization_format: SerializationFormat::Json,
        }
    }
}

/// ビルド設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSettings {
    pub game_name: String,
    pub main_scene: String,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub assets: Vec<String>,
}

impl Default for BuildSettings {
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
