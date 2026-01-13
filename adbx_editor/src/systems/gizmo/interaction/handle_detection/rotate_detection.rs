use super::super::super::draw::calculate_gizmo_scale;
use super::super::super::resource::GizmoHandle;
use super::super::ray_intersection::ray_circle_intersection;
use bevy::prelude::*;

/// Rotateハンドルの検出
pub fn detect_rotate_handle(
    ray_origin: Vec3,
    ray_direction: Vec3,
    transform: &Transform,
    camera_transform: &GlobalTransform,
    camera_query: &Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
) -> Option<GizmoHandle> {
    let scale = calculate_gizmo_scale(transform, camera_query);
    let ring_radius = 1.0 * scale;
    let ring_thickness = 0.1 * scale;

    // カメラから見た方向を計算
    let camera_pos = camera_transform.translation();
    let _to_camera = (camera_pos - transform.translation).normalize();

    let handles = [
        (*transform.local_x(), GizmoHandle::RotateX),
        (*transform.local_y(), GizmoHandle::RotateY),
        (*transform.local_z(), GizmoHandle::RotateZ),
    ];

    let mut closest_handle: Option<(f32, GizmoHandle)> = None;

    for (axis_dir, handle_type) in handles.iter() {
        // リングの中心と法線
        let ring_center = transform.translation;
        let ring_normal = *axis_dir;

        // リングの半径範囲内で交差判定
        let inner_radius = ring_radius - ring_thickness;
        let outer_radius = ring_radius + ring_thickness;

        if let Some(t) = ray_circle_intersection(
            ray_origin,
            ray_direction,
            ring_center,
            ring_normal,
            outer_radius,
        ) {
            let intersection = ray_origin + ray_direction * t;
            let dist_from_center = (intersection - ring_center).length();

            if dist_from_center >= inner_radius && dist_from_center <= outer_radius {
                if let Some((best_t, _)) = closest_handle {
                    if t < best_t {
                        closest_handle = Some((t, *handle_type));
                    }
                } else {
                    closest_handle = Some((t, *handle_type));
                }
            }
        }
    }

    closest_handle.map(|(_, handle)| handle)
}
