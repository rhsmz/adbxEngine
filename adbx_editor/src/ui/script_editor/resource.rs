use bevy::prelude::*;
use std::path::PathBuf;

/// スクリプトエディタのリソース
#[derive(Resource, Default)]
pub struct ScriptEditor {
    pub current_script: Option<PathBuf>,
    pub content: String,
    pub entity: Option<Entity>,
    pub content_entity: Option<Entity>, // 現在のコンテンツエンティティ
    pub text_area_entity: Option<Entity>, // テキストエリアエンティティ
    pub errors: Vec<ScriptError>,       // エラー情報
    pub cursor_position: usize,         // カーソル位置（文字数）
    pub selection_start: Option<usize>, // 選択範囲の開始位置
    pub is_focused: bool,               // エディタがフォーカスされているか
    pub edit_history: Vec<String>,      // Undo/Redo用の履歴
    pub history_index: usize,           // 履歴の現在位置
    pub should_execute: bool,           // スクリプト実行リクエスト
}

/// スクリプトエラー情報
#[derive(Debug, Clone)]
pub struct ScriptError {
    pub line: usize,
    pub message: String,
    pub column: Option<usize>,
}
