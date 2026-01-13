use bevy::prelude::*;
use super::super::resource::GizmoHandle;

/// Gizmo操作に基づいてTransformを更新
pub fn update_transform_from_gizmo(
    transform: &mut Transform,
    handle: GizmoHandle,
    mouse_delta: Vec2,
    _start_transform: Option<&Transform>,
) {
    let sensitivity = 0.01;
    
    match handle {
        GizmoHandle::TranslateX => {
            transform.translation += transform.local_x() * mouse_delta.x * sensitivity;
        }
        GizmoHandle::TranslateY => {
            transform.translation += transform.local_y() * mouse_delta.y * sensitivity;
        }
        GizmoHandle::TranslateZ => {
            transform.translation += transform.local_z() * mouse_delta.x * sensitivity;
        }
        GizmoHandle::RotateX => {
            let rotation_delta = mouse_delta.x * sensitivity;
            transform.rotate_local_x(rotation_delta);
        }
        GizmoHandle::RotateY => {
            let rotation_delta = mouse_delta.y * sensitivity;
            transform.rotate_local_y(rotation_delta);
        }
        GizmoHandle::RotateZ => {
            let rotation_delta = mouse_delta.x * sensitivity;
            transform.rotate_local_z(rotation_delta);
        }
        GizmoHandle::ScaleX => {
            let scale_delta = mouse_delta.x * sensitivity;
            transform.scale.x = (transform.scale.x + scale_delta).max(0.01);
        }
        GizmoHandle::ScaleY => {
            let scale_delta = mouse_delta.y * sensitivity;
            transform.scale.y = (transform.scale.y + scale_delta).max(0.01);
        }
        GizmoHandle::ScaleZ => {
            let scale_delta = mouse_delta.x * sensitivity;
            transform.scale.z = (transform.scale.z + scale_delta).max(0.01);
        }
    }
}
