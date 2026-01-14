use crate::error::EditorError;
use bevy::prelude::*;

/// エラーダイアログのリソース
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct ErrorDialog {
    pub is_visible: bool,
    pub title: String,
    pub message: String,
    pub details: Option<String>, // 詳細情報（展開可能）
    pub error_type: ErrorType,
    pub dialog_entity: Option<Entity>, // ダイアログエンティティ
    pub error: Option<EditorError>,    // エラーオブジェクト（構造化されたエラー情報）
    pub show_details: bool,            // 詳細情報を表示するかどうか
}

/// エラーの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Error,   // エラー（赤）
    Warning, // 警告（黄）
    Info,    // 情報（青）
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Error
    }
}
