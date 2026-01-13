use bevy::prelude::*;
use super::super::resource::{GizmoInteraction, GizmoHandle};
use super::utils::{calculate_gizmo_scale, get_handle_color};
use super::rotation_ring::draw_rotation_ring;

/// 回転用Gizmoの描画
pub fn draw_rotate_gizmo(
    gizmos: &mut Gizmos,
    transform: &Transform,
    gizmo_interaction: &GizmoInteraction,
    camera_query: &Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
    _windows: &Query<&Window>,
) {
    // カメラ距離に応じたスケーリング
    let scale = calculate_gizmo_scale(transform, camera_query);
    let radius = 1.0 * scale;
    let segments = 32;
    
    // X軸回転（赤）
    let x_axis = *transform.local_x();
    let x_color = get_handle_color(gizmo_interaction, GizmoHandle::RotateX, 1.0, 0.0, 0.0);
    draw_rotation_ring(gizmos, transform, x_axis, radius, segments, x_color);
    
    // Y軸回転（緑）
    let y_axis = *transform.local_y();
    let y_color = get_handle_color(gizmo_interaction, GizmoHandle::RotateY, 0.0, 1.0, 0.0);
    draw_rotation_ring(gizmos, transform, y_axis, radius, segments, y_color);
    
    // Z軸回転（青）
    let z_axis = *transform.local_z();
    let z_color = get_handle_color(gizmo_interaction, GizmoHandle::RotateZ, 0.0, 0.0, 1.0);
    draw_rotation_ring(gizmos, transform, z_axis, radius, segments, z_color);
}
