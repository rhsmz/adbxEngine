use bevy::prelude::*;
use super::super::super::resource::GizmoHandle;
use super::super::super::draw::calculate_gizmo_scale;
use super::super::ray_intersection::ray_sphere_intersection;

/// Scaleハンドルの検出
pub fn detect_scale_handle(
    ray_origin: Vec3,
    ray_direction: Vec3,
    transform: &Transform,
    camera_query: &Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
) -> Option<GizmoHandle> {
    let scale = calculate_gizmo_scale(transform, camera_query);
    let handle_length = 1.0 * scale;
    let handle_size = 0.15 * scale; // ハンドル先端の球のサイズ
    
    let handles = [
        (*transform.local_x(), GizmoHandle::ScaleX),
        (*transform.local_y(), GizmoHandle::ScaleY),
        (*transform.local_z(), GizmoHandle::ScaleZ),
    ];
    
    let mut closest_handle: Option<(f32, GizmoHandle)> = None;
    
    for (axis_dir, handle_type) in handles.iter() {
        let handle_end = transform.translation + *axis_dir * handle_length;
        
        if let Some(t) = ray_sphere_intersection(ray_origin, ray_direction, handle_end, handle_size) {
            if let Some((best_t, _)) = closest_handle {
                if t < best_t {
                    closest_handle = Some((t, *handle_type));
                }
            } else {
                closest_handle = Some((t, *handle_type));
            }
        }
    }
    
    closest_handle.map(|(_, handle)| handle)
}
