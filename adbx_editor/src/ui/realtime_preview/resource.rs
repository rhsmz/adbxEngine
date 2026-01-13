use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// リアルタイムプレビューのリソース
#[derive(Resource)]
pub struct RealtimePreview {
    pub is_enabled: bool,
    pub preview_window_entity: Option<Entity>,
    pub preview_content: HashMap<String, PreviewContent>, // ファイルパス -> プレビューコンテンツ
    pub last_update_time: HashMap<String, Instant>, // ファイルパス -> 最終更新時刻
    pub update_throttle: Duration, // 更新のスロットル時間（デフォルト: 100ms）
}

/// プレビューコンテンツ
#[derive(Debug, Clone)]
pub struct PreviewContent {
    pub content: String,
    pub rendered_content: Option<String>, // レンダリングされたコンテンツ（例: HTML、画像など）
    pub preview_type: PreviewType,
    pub error: Option<String>, // プレビュー生成時のエラー
}

/// プレビューの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewType {
    Code, // コードのプレビュー（シンタックスハイライト済み）
    Html, // HTMLのプレビュー
    Image, // 画像のプレビュー
    Scene, // シーンのプレビュー
    Script, // スクリプトの実行結果
}

impl Default for RealtimePreview {
    fn default() -> Self {
        Self {
            is_enabled: true,
            preview_window_entity: None,
            preview_content: HashMap::new(),
            last_update_time: HashMap::new(),
            update_throttle: Duration::from_millis(100), // 100msごとに更新
        }
    }
}

impl RealtimePreview {
    /// プレビューを取得
    pub fn get_preview(&self, file_path: &str) -> Option<&PreviewContent> {
        self.preview_content.get(file_path)
    }
}
