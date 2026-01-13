use super::super::super::resource::GizmoHandle;
use super::rotate_detection::detect_rotate_handle;
use super::scale_detection::detect_scale_handle;
use super::translate_detection::detect_translate_handle;
use crate::systems::selection::Selection;
use crate::ui::scene_view::{GizmoMode, SceneView};
use bevy::prelude::*;

/// Gizmoハンドルのクリック検出（正確なレイキャスト実装）
pub fn detect_gizmo_handle_click(
    selection: &Selection,
    scene_view: &SceneView,
    windows: Query<&Window>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
    transform_query: Query<&Transform>,
) -> Option<GizmoHandle> {
    if selection.selected_entities.is_empty() {
        return None;
    }

    let Ok(window) = windows.single() else {
        return None;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return None;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return None;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return None;
    };

    if let Some(&entity) = selection.selected_entities.first() {
        if let Ok(transform) = transform_query.get(entity) {
            match scene_view.gizmo_mode {
                GizmoMode::Translate => {
                    return detect_translate_handle(
                        ray.origin,
                        ray.direction.normalize(),
                        transform,
                        &camera_query,
                    );
                }
                GizmoMode::Rotate => {
                    return detect_rotate_handle(
                        ray.origin,
                        ray.direction.normalize(),
                        transform,
                        camera_transform,
                        &camera_query,
                    );
                }
                GizmoMode::Scale => {
                    return detect_scale_handle(
                        ray.origin,
                        ray.direction.normalize(),
                        transform,
                        &camera_query,
                    );
                }
                _ => {}
            }
        }
    }

    None
}
