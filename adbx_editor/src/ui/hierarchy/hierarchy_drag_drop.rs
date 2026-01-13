use bevy::prelude::*;
use crate::ui::hierarchy::hierarchy_view_resource::{HierarchyDragState, HierarchyItem};

/// エンティティが別のエンティティの子孫かどうかを判定
pub fn is_entity_descendant_of_entity(
    potential_ancestor: Entity,
    potential_descendant: Entity,
    children_query: &Query<&Children>,
) -> bool {
    if let Ok(children) = children_query.get(potential_ancestor) {
        for child in children.iter() {
            if child == potential_descendant {
                return true;
            }
            if is_entity_descendant_of_entity(child, potential_descendant, children_query) {
                return true;
            }
        }
    }
    false
}

/// ドラッグ&ドロップの処理
pub fn handle_hierarchy_view_drag_and_drop(
    mut commands: Commands,
    mut drag_state: ResMut<HierarchyDragState>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    hierarchy_item_query: Query<&HierarchyItem>,
    children_query: Query<&Children>,
) {
    // ドラッグ中のドロップ先検出
    if let Some(dragging_entity) = drag_state.dragging_entity {
        if mouse_input.pressed(MouseButton::Left) {
            if let Ok(window) = windows.single() {
                if window.cursor_position().is_some() {
                    let mut closest_target: Option<Entity> = None;
                    let _closest_distance = f32::MAX;
                    
                    // マウス位置がHierarchyItemの範囲内にあるかチェック
                    // 簡易実装: すべてのHierarchyItemをドロップ先候補とする
                    for hierarchy_item in hierarchy_item_query.iter() {
                        // 自分自身はドロップ先にしない
                        if hierarchy_item.entity == dragging_entity {
                            continue;
                        }
                        
                        // 子孫エンティティはドロップ先にしない（循環参照を防ぐ）
                        if is_entity_descendant_of_entity(hierarchy_item.entity, dragging_entity, &children_query) {
                            continue;
                        }
                        
                        // 簡易実装: 最初に見つかった有効なエンティティをドロップ先とする
                        closest_target = Some(hierarchy_item.entity);
                        break;
                    }
                    
                    drag_state.hovered_drop_target = closest_target;
                }
            }
        } else {
            // ドロップ処理
            if let Some(drop_target) = drag_state.hovered_drop_target {
                // 新しい親子関係を設定
                // 注意: Bevyは自動的に既存の親子関係を解除してから新しい親子関係を設定します
                commands.entity(drop_target).add_child(dragging_entity);
            }
            
            drag_state.dragging_entity = None;
            drag_state.drag_start_pos = None;
            drag_state.hovered_drop_target = None;
        }
    }
}
