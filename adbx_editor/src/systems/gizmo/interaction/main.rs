use bevy::prelude::*;
use crate::systems::selection::Selection;
use crate::ui::scene_view::{SceneView, is_cursor_in_scene_view_area};
use super::super::resource::GizmoInteraction;
use super::handle_detection::detect_gizmo_handle_click;
use super::transform_update::update_transform_from_gizmo;

/// Gizmoハンドルの選択とドラッグ処理
pub fn handle_gizmo_interaction(
    mut gizmo_interaction: ResMut<GizmoInteraction>,
    selection: ResMut<Selection>,
    scene_view: Res<SceneView>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    _mouse_motion_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseMotion>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
    mut transform_queries: ParamSet<(
        Query<&Transform>,
        Query<&mut Transform>,
    )>,
) {
    // シーンビューエリア内でのみGizmo操作を有効にする
    if !is_cursor_in_scene_view_area(&windows, &camera_query) {
        // エリア外ではホバー状態をクリア
        gizmo_interaction.hovered_handle = None;
        return;
    }
    // マウスドラッグの処理
    if let Some(handle) = gizmo_interaction.active_handle {
        if mouse_input.pressed(MouseButton::Left) {
            // ドラッグ中
            if let Some(window) = windows.iter().next() {
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Some(start_pos) = gizmo_interaction.drag_start_pos {
                        let delta = cursor_pos - start_pos;
                        
                        // 選択されたエンティティのTransformを更新
                        // ミュータブルクエリを使用
                        for &entity in selection.selected_entities.iter() {
                            if let Ok(mut transform) = transform_queries.p1().get_mut(entity) {
                                update_transform_from_gizmo(
                                    &mut transform,
                                    handle,
                                    delta,
                                    gizmo_interaction.drag_start_transform.as_ref(),
                                );
                            }
                        }
                    }
                }
            }
        } else {
            // ドラッグ終了
            gizmo_interaction.active_handle = None;
            gizmo_interaction.drag_start_pos = None;
            gizmo_interaction.drag_start_transform = None;
        }
    } else if mouse_input.just_pressed(MouseButton::Left) {
        // 新しいドラッグ開始
        if scene_view.gizmo_mode != crate::ui::scene_view::GizmoMode::None {
            // 読み取り専用クエリを使用
            let handle = detect_gizmo_handle_click(
                &selection,
                &scene_view,
                windows,
                camera_query,
                transform_queries.p0(),
            );
            
            if let Some(handle) = handle {
                gizmo_interaction.active_handle = Some(handle);
                if let Some(window) = windows.iter().next() {
                    gizmo_interaction.drag_start_pos = window.cursor_position();
                }
                // 読み取り専用クエリを使用してdrag_start_transformを取得
                if let Some(&entity) = selection.selected_entities.first() {
                    if let Ok(transform) = transform_queries.p0().get(entity) {
                        gizmo_interaction.drag_start_transform = Some(*transform);
                    }
                }
            }
        }
    }
    
    // ホバー中のハンドルを検出（ドラッグ中でない場合のみ）
    if gizmo_interaction.active_handle.is_none() {
        // 読み取り専用クエリを使用
        gizmo_interaction.hovered_handle = detect_gizmo_handle_click(
            &selection,
            &scene_view,
            windows,
            camera_query,
            transform_queries.p0(),
        );
    } else {
        gizmo_interaction.hovered_handle = None;
    }
}
