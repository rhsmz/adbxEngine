use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::ui::code_editor::state::{OpenFile, CompletionState};
use crate::ui::code_editor::ai_integration::{AiRequest, AiResponse};

/// ハイブコーディングエディタのリソース
#[derive(Resource, Default)]
pub struct CodeEditor {
    pub current_file: Option<String>,
    pub content: String,
    pub ai_enabled: bool,
    pub ai_requests: VecDeque<AiRequest>,
    pub ai_responses: VecDeque<AiResponse>,
    pub open_files: Vec<OpenFile>, // 開いているファイルのリスト
    pub active_tab: usize, // アクティブなタブのインデックス
    pub content_entity: Option<Entity>, // 現在のコンテンツエンティティ
    pub text_area_entity: Option<Entity>, // テキストエリアエンティティ
    pub cursor_position: usize, // カーソル位置（文字数）
    pub selection_start: Option<usize>, // 選択範囲の開始位置
    pub is_focused: bool, // エディタがフォーカスされているか
    pub edit_history: Vec<String>, // Undo/Redo用の履歴
    pub history_index: usize, // 履歴の現在位置
    pub completion_state: CompletionState, // コード補完の状態
    #[cfg(feature = "ai")]
    pub ai_progress: Option<String>, // AI処理の進行状況
    pub scroll_offset: f32, // スクロール位置（行単位）
    pub line_height: f32, // 1行の高さ（ピクセル）
    pub visible_lines: usize, // 表示可能な行数
    // 変更検知用のフィールド
    pub last_content_hash: u64, // 前回のコンテンツのハッシュ
    pub last_cursor_position: usize, // 前回のカーソル位置
    pub last_scroll_offset: f32, // 前回のスクロール位置
    pub last_active_tab: usize, // 前回のアクティブタブ
    pub last_completion_visible: bool, // 前回の補完表示状態
    pub syntax_highlight_cache: std::collections::HashMap<usize, Vec<(String, Color)>>, // 行ごとのハイライトキャッシュ（行番号 -> トークンリスト）
    pub syntax_highlight_cache_hash: u64, // キャッシュのハッシュ（コンテンツが変更されたかどうかを判定）
}
