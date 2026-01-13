use super::super::resource::{GizmoHandle, GizmoInteraction};
use super::utils::{calculate_gizmo_scale, get_handle_color};
use bevy::prelude::*;

/// 移動用Gizmoの描画
pub fn draw_translate_gizmo(
    gizmos: &mut Gizmos,
    transform: &Transform,
    gizmo_interaction: &GizmoInteraction,
    camera_query: &Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
    _windows: &Query<&Window>,
) {
    // カメラ距離に応じたスケーリング
    let scale = calculate_gizmo_scale(transform, camera_query);
    let handle_size = 0.3 * scale;
    let handle_length = 1.0 * scale;

    // X軸（赤）
    let x_end = transform.translation + transform.local_x() * handle_length;
    let x_color = get_handle_color(gizmo_interaction, GizmoHandle::TranslateX, 1.0, 0.0, 0.0);
    gizmos.line(transform.translation, x_end, x_color);
    gizmos.sphere(x_end, handle_size, x_color);

    // Y軸（緑）
    let y_end = transform.translation + transform.local_y() * handle_length;
    let y_color = get_handle_color(gizmo_interaction, GizmoHandle::TranslateY, 0.0, 1.0, 0.0);
    gizmos.line(transform.translation, y_end, y_color);
    gizmos.sphere(y_end, handle_size, y_color);

    // Z軸（青）
    let z_end = transform.translation + transform.local_z() * handle_length;
    let z_color = get_handle_color(gizmo_interaction, GizmoHandle::TranslateZ, 0.0, 0.0, 1.0);
    gizmos.line(transform.translation, z_end, z_color);
    gizmos.sphere(z_end, handle_size, z_color);
}
