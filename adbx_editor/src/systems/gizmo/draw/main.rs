use super::super::resource::GizmoInteraction;
use super::rotate::draw_rotate_gizmo;
use super::scale::draw_scale_gizmo;
use super::translate::draw_translate_gizmo;
use super::utils::draw_axis_gizmo;
use crate::systems::selection::Selection;
use crate::ui::scene_view::{GizmoMode, SceneView};
use bevy::prelude::*;

/// Gizmoシステム（高度版）
pub fn draw_gizmos(
    mut gizmos: Gizmos,
    selection: Res<Selection>,
    scene_view: Res<SceneView>,
    gizmo_interaction: Res<GizmoInteraction>,
    transform_query: Query<&Transform>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
    windows: Query<&Window>,
) {
    // 選択されたEntityのGizmoを描画
    for &entity in selection.selected_entities.iter() {
        if let Ok(transform) = transform_query.get(entity) {
            match scene_view.gizmo_mode {
                GizmoMode::Translate => {
                    draw_translate_gizmo(
                        &mut gizmos,
                        transform,
                        &gizmo_interaction,
                        &camera_query,
                        &windows,
                    );
                }
                GizmoMode::Rotate => {
                    draw_rotate_gizmo(
                        &mut gizmos,
                        transform,
                        &gizmo_interaction,
                        &camera_query,
                        &windows,
                    );
                }
                GizmoMode::Scale => {
                    draw_scale_gizmo(
                        &mut gizmos,
                        transform,
                        &gizmo_interaction,
                        &camera_query,
                        &windows,
                    );
                }
                GizmoMode::None => {
                    // 基本的な軸表示のみ
                    draw_axis_gizmo(&mut gizmos, transform);
                }
            }
        }
    }
}
