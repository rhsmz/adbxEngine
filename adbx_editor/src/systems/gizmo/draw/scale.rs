use bevy::prelude::*;
use super::super::resource::{GizmoInteraction, GizmoHandle};
use super::utils::get_handle_color;

/// スケール用Gizmoの描画
pub fn draw_scale_gizmo(
    gizmos: &mut Gizmos,
    transform: &Transform,
    gizmo_interaction: &GizmoInteraction,
    _camera_query: &Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
    _windows: &Query<&Window>,
) {
    let handle_size = 0.2;
    let handle_length = 1.0;
    
    // X軸（赤）
    let x_end = transform.translation + transform.local_x() * handle_length;
    let x_color = get_handle_color(gizmo_interaction, GizmoHandle::ScaleX, 1.0, 0.0, 0.0);
    gizmos.line(
        transform.translation,
        x_end,
        x_color,
    );
    gizmos.cuboid(
        Transform::from_translation(x_end)
            .with_scale(Vec3::splat(handle_size)),
        x_color,
    );
    
    // Y軸（緑）
    let y_end = transform.translation + transform.local_y() * handle_length;
    let y_color = get_handle_color(gizmo_interaction, GizmoHandle::ScaleY, 0.0, 1.0, 0.0);
    gizmos.line(
        transform.translation,
        y_end,
        y_color,
    );
    gizmos.cuboid(
        Transform::from_translation(y_end)
            .with_scale(Vec3::splat(handle_size)),
        y_color,
    );
    
    // Z軸（青）
    let z_end = transform.translation + transform.local_z() * handle_length;
    let z_color = get_handle_color(gizmo_interaction, GizmoHandle::ScaleZ, 0.0, 0.0, 1.0);
    gizmos.line(
        transform.translation,
        z_end,
        z_color,
    );
    gizmos.cuboid(
        Transform::from_translation(z_end)
            .with_scale(Vec3::splat(handle_size)),
        z_color,
    );
}
