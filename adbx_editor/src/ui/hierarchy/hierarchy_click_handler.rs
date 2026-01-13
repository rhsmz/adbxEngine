use crate::systems::selection::Selection;
use crate::ui::hierarchy::hierarchy_view_resource::{
    HierarchyDragState, HierarchyItem, HierarchyView,
};
use bevy::prelude::*;

/// ヒエラルキーアイテムのクリック処理
pub fn handle_hierarchy_view_item_click(
    mut hierarchy_view: ResMut<HierarchyView>,
    mut selection: ResMut<Selection>,
    mut drag_state: ResMut<HierarchyDragState>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseMotion>,
    windows: Query<&Window>,
    interaction_query: Query<(&Interaction, &HierarchyItem), Changed<Interaction>>,
    children_query: Query<&Children>,
) {
    // ドラッグ開始の検出
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, hierarchy_item) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                // エンティティを選択
                selection.selected_entities.clear();
                selection.selected_entities.push(hierarchy_item.entity);

                // ドラッグ開始位置を記録
                if let Ok(window) = windows.single() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        drag_state.dragging_entity = Some(hierarchy_item.entity);
                        drag_state.drag_start_pos = Some(cursor_pos);
                    }
                }

                // 展開/折りたたみの切り替え（ドラッグでない場合のみ）
                // ドラッグ判定は後で行うため、ここでは展開/折りたたみを実行しない
            }
        }
    }

    // ドラッグ中の処理
    if drag_state.dragging_entity.is_some() && mouse_input.pressed(MouseButton::Left) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        // ドラッグ開始位置から一定距離以上移動した場合、ドラッグと判定
        if let Some(start_pos) = drag_state.drag_start_pos {
            if let Ok(window) = windows.single() {
                if let Some(current_pos) = window.cursor_position() {
                    let drag_distance = (current_pos - start_pos).length();
                    if drag_distance > 5.0 {
                        // ドラッグ中
                        // ドロップ先の検出は別のシステムで行う
                    }
                }
            }
        }
    }

    // ドラッグ終了の検出
    if mouse_input.just_released(MouseButton::Left) {
        if drag_state.dragging_entity.is_some() {
            // ドロップ処理は別のシステムで行う
            drag_state.dragging_entity = None;
            drag_state.drag_start_pos = None;
        } else {
            // ドラッグでない場合は展開/折りたたみを実行
            for (interaction, hierarchy_item) in interaction_query.iter() {
                if *interaction == Interaction::None {
                    if children_query.get(hierarchy_item.entity).is_ok() {
                        if hierarchy_view
                            .expanded_entities
                            .contains(&hierarchy_item.entity)
                        {
                            hierarchy_view
                                .expanded_entities
                                .remove(&hierarchy_item.entity);
                        } else {
                            hierarchy_view
                                .expanded_entities
                                .insert(hierarchy_item.entity);
                        }
                    }
                }
            }
        }
    }
}
