use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// エディタ設定のリソース
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub theme: EditorTheme,
    pub window_size: (u32, u32),
    pub auto_save: bool,
    pub auto_save_interval: u32, // 秒
    pub show_line_numbers: bool,
    pub font_size: f32,
    pub ai_provider: String,
    pub ai_api_key: Option<String>,
    // 追加設定項目
    pub tab_width: u32, // タブ幅（スペース数）
    pub word_wrap: bool, // ワードラップ
    pub show_minimap: bool, // ミニマップ表示
    pub enable_auto_complete: bool, // 自動補完
    pub enable_syntax_highlight: bool, // シンタックスハイライト
    pub editor_language: String, // エディタの言語（ja, enなど）
    pub max_undo_history: usize, // Undo履歴の最大数
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            theme: EditorTheme::Dark,
            window_size: (1920, 1080),
            auto_save: false,
            auto_save_interval: 300,
            show_line_numbers: true,
            font_size: 11.0,
            ai_provider: "Claude".to_string(),
            ai_api_key: None,
            tab_width: 4,
            word_wrap: false,
            show_minimap: false,
            enable_auto_complete: true,
            enable_syntax_highlight: true,
            editor_language: "ja".to_string(),
            max_undo_history: 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorTheme {
    Dark,
    Light,
}

/// 設定UIパネルのリソース
#[derive(Resource, Default)]
pub struct SettingsPanel {
    pub is_open: bool,
    pub content_entity: Option<Entity>,
}
