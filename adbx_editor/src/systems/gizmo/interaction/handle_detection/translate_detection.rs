use bevy::prelude::*;
use super::super::super::resource::GizmoHandle;
use super::super::super::draw::calculate_gizmo_scale;
use super::super::ray_intersection::ray_to_line_distance;

/// Translateハンドルの検出
pub fn detect_translate_handle(
    ray_origin: Vec3,
    ray_direction: Vec3,
    transform: &Transform,
    camera_query: &Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
) -> Option<GizmoHandle> {
    let scale = calculate_gizmo_scale(transform, camera_query);
    let handle_length = 1.0 * scale;
    let handle_thickness = 0.05 * scale; // ハンドルの太さ
    
    let handles = [
        (*transform.local_x(), GizmoHandle::TranslateX),
        (*transform.local_y(), GizmoHandle::TranslateY),
        (*transform.local_z(), GizmoHandle::TranslateZ),
    ];
    
    let mut closest_handle: Option<(f32, GizmoHandle)> = None;
    
    for (axis_dir, handle_type) in handles.iter() {
        let line_start = transform.translation;
        let line_end = transform.translation + *axis_dir * handle_length;
        
        let distance = ray_to_line_distance(ray_origin, ray_direction, line_start, line_end);
        
        if distance < handle_thickness {
            if let Some((best_dist, _)) = closest_handle {
                if distance < best_dist {
                    closest_handle = Some((distance, *handle_type));
                }
            } else {
                closest_handle = Some((distance, *handle_type));
            }
        }
    }
    
    closest_handle.map(|(_, handle)| handle)
}
