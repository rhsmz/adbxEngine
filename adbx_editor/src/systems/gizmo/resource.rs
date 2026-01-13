use bevy::prelude::*;

/// Gizmo操作のリソース
#[derive(Resource, Default)]
pub struct GizmoInteraction {
    pub active_handle: Option<GizmoHandle>,
    pub hovered_handle: Option<GizmoHandle>,
    pub drag_start_pos: Option<Vec2>,
    pub drag_start_transform: Option<Transform>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoHandle {
    TranslateX,
    TranslateY,
    TranslateZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}
