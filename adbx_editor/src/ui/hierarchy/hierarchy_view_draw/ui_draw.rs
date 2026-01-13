use bevy::prelude::*;
use std::collections::HashSet;
use super::super::hierarchy_view_resource::{HierarchyView, HierarchyDragState, HierarchyItem};
use super::entity_collection::{collect_expanded_hierarchy_entities, calculate_hierarchy_item_indent_level};
use super::item_spawn::spawn_hierarchy_view_item;
use crate::systems::selection::Selection;

/// ヒエラルキービューのUI描画（仮想スクロール対応版）
pub fn draw_hierarchy_view_ui(
    mut commands: Commands,
    mut hierarchy_view: ResMut<HierarchyView>,
    selection: Res<Selection>,
    drag_state: Res<HierarchyDragState>,
    root_query: Query<Entity, With<Name>>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
    hierarchy_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    hierarchy_item_query: Query<(Entity, &HierarchyItem)>,
    windows: Query<&Window>,
) {
    // ヒエラルキーパネルを検索
    let panel_opt = hierarchy_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "HierarchyPanel"
        } else {
            false
        }
    });
    
    if let Some(panel) = panel_opt {
        // 現在のエンティティセットを取得
        let root_entities: Vec<Entity> = root_query.iter().collect();
        let current_entities: HashSet<Entity> = root_entities.iter().copied().collect();
        
        // 表示可能なエンティティのリストを収集（展開された状態で）
        let visible_entities = collect_expanded_hierarchy_entities(&root_entities, &hierarchy_view, &children_query, &name_query);
        hierarchy_view.total_items = visible_entities.len();
        
        // パネルの高さを取得（仮想スクロールの計算に使用）
        let panel_height = if let Ok(window) = windows.single() {
            window.height() * 0.5 // 簡易実装：ウィンドウの高さの50%をパネル高さとする
        } else {
            400.0 // デフォルト値
        };
        
        // 1アイテムの高さ
        hierarchy_view.item_height = 20.0;
        
        // 表示可能なアイテム数
        let visible_item_count = (panel_height / hierarchy_view.item_height).ceil() as usize;
        
        // スクロール位置に基づいて表示範囲を計算
        let scroll_index = (hierarchy_view.scroll_offset / hierarchy_view.item_height).floor() as usize;
        let start_index = scroll_index.min(visible_entities.len().saturating_sub(1));
        let end_index = (start_index + visible_item_count + 2).min(visible_entities.len()); // +2はバッファ
        
        hierarchy_view.visible_range = (start_index, end_index);
        
        // 表示範囲内のエンティティのみを処理
        let visible_range_entities: HashSet<Entity> = visible_entities[start_index..end_index]
            .iter()
            .copied()
            .collect();
        
        // 削除されたエンティティと追加されたエンティティを先に収集
        let removed_entities: Vec<Entity> = hierarchy_view.last_entities.difference(&current_entities).copied().collect();
        let _new_entities: Vec<Entity> = current_entities.difference(&hierarchy_view.last_entities).copied().collect();
        
        // 表示範囲外になったエンティティのUIを削除
        for (entity, ui_entity) in hierarchy_view.ui_entity_map.iter() {
            if !visible_range_entities.contains(entity) {
                if let Ok(mut entity_commands) = commands.get_entity(*ui_entity) {
                    entity_commands.despawn();
                }
            }
        }
        hierarchy_view.ui_entity_map.retain(|entity, _| visible_range_entities.contains(entity));
        
        // 削除されたエンティティのUIを削除
        for removed_entity in removed_entities {
            if let Some(ui_entity) = hierarchy_view.ui_entity_map.remove(&removed_entity) {
                if let Ok(mut entity_commands) = commands.get_entity(ui_entity) {
                    entity_commands.despawn();
                }
            }
        }
        
        // 表示範囲内の新しいエンティティのUIを作成
        for (index, entity) in visible_entities[start_index..end_index].iter().enumerate() {
            if !hierarchy_view.ui_entity_map.contains_key(entity) {
                if let Ok(name) = name_query.get(*entity) {
                    // インデントレベルを計算（簡易実装：親エンティティの数を数える）
                    let indent = calculate_hierarchy_item_indent_level(*entity, &root_entities, &children_query, &hierarchy_view);
                    
                    let ui_entity = spawn_hierarchy_view_item(
                        &mut commands,
                        *entity,
                        name,
                        &hierarchy_view,
                        &selection,
                        &*drag_state,
                        &children_query,
                        &name_query,
                        panel,
                        indent,
                        start_index + index, // 表示位置のインデックス
                    );
                    hierarchy_view.ui_entity_map.insert(*entity, ui_entity);
                }
            }
        }
        
        // 既存のUI要素を更新（選択状態、ドラッグ状態の変更など）
        for (ui_entity, hierarchy_item) in hierarchy_item_query.iter() {
            let entity = hierarchy_item.entity;
            if !visible_range_entities.contains(&entity) {
                continue; // 表示範囲外のエンティティは更新しない
            }
            
            let is_selected = selection.selected_entities.contains(&entity);
            let is_dragging = drag_state.dragging_entity == Some(entity);
            let is_drop_target = drag_state.hovered_drop_target == Some(entity);
            
            // 背景色を更新
            let bg_color = if is_dragging {
                Color::srgb(0.4, 0.6, 0.8) // ドラッグ中は明るい青
            } else if is_drop_target {
                Color::srgb(0.6, 0.8, 0.4) // ドロップ先は明るい緑
            } else if is_selected {
                Color::srgb(0.2, 0.4, 0.6) // 選択中は青
            } else {
                Color::NONE
            };
            
            if let Ok(mut entity_commands) = commands.get_entity(ui_entity) {
                entity_commands.insert(BackgroundColor(bg_color));
            }
        }
        
        // 現在のエンティティセットを保存
        hierarchy_view.last_entities = current_entities;
        hierarchy_view.needs_rebuild = false;
    }
}
