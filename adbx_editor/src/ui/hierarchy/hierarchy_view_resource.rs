use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// ヒエラルキービューのリソース
#[derive(Resource, Default)]
pub struct HierarchyView {
    pub expanded_entities: HashSet<Entity>,
    pub last_entities: HashSet<Entity>, // 前回のフレームで表示されていたエンティティ
    pub ui_entity_map: HashMap<Entity, Entity>, // エンティティ -> UIエンティティのマッピング
    pub scroll_offset: f32,             // スクロール位置（ピクセル単位）
    pub visible_range: (usize, usize),  // 表示範囲（開始インデックス、終了インデックス）
    pub item_height: f32,               // 1アイテムの高さ（ピクセル単位）
    pub total_items: usize,             // 総アイテム数（展開された状態での表示可能なアイテム数）
    pub needs_rebuild: bool,            // UIの再構築が必要かどうか
}

/// ドラッグ&ドロップ状態を管理するリソース
#[derive(Resource, Default)]
pub struct HierarchyDragState {
    pub dragging_entity: Option<Entity>, // ドラッグ中のエンティティ
    pub drag_start_pos: Option<Vec2>,    // ドラッグ開始位置
    pub hovered_drop_target: Option<Entity>, // ホバー中のドロップ先エンティティ
}

/// ヒエラルキーアイテムのマーカーコンポーネント
#[derive(Component)]
pub struct HierarchyItem {
    pub entity: Entity,
}
