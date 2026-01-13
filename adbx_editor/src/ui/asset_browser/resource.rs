use bevy::prelude::*;
use std::path::PathBuf;

/// アセットブラウザーのリソース
#[derive(Resource)]
pub struct AssetBrowser {
    pub current_path: PathBuf,
    pub selected_asset: Option<PathBuf>,
    pub asset_files: Vec<AssetFileInfo>,
    pub content_entity: Option<Entity>, // 現在のコンテンツエンティティ（再描画の最適化用）
    pub image_handles: std::collections::HashMap<PathBuf, Handle<Image>>, // 画像ハンドルのキャッシュ
    pub pending_image_updates: bool,   // 画像の読み込み待ちがあるか
    pub is_export_pending: bool,       // エクスポート処理待ちかどうか
    pub scroll_offset: f32,            // スクロール位置（ピクセル単位）
    pub visible_range: (usize, usize), // 表示範囲（開始インデックス、終了インデックス）
    pub item_height: f32,              // 1アイテムの高さ（ピクセル単位）
    pub total_items: usize,            // 総アイテム数
}

#[derive(Debug, Clone)]
pub struct AssetFileInfo {
    pub path: PathBuf,
    pub name: String,
    pub asset_type: AssetType,
    pub is_directory: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Mesh,
    Texture,
    Material,
    Script,
    Scene,
    Text,
    Other,
}

impl AssetBrowser {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::from("assets"),
            selected_asset: None,
            asset_files: Vec::new(),
            content_entity: None,
            image_handles: std::collections::HashMap::new(),
            pending_image_updates: false,
            is_export_pending: false,
            scroll_offset: 0.0,
            visible_range: (0, 0),
            item_height: 30.0,
            total_items: 0,
        }
    }
}

impl Default for AssetBrowser {
    fn default() -> Self {
        Self::new()
    }
}
