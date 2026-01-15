use bevy::prelude::*;

/// シーンビューのリソース
#[derive(Resource)]
#[allow(dead_code)]
pub struct SceneView {
    pub camera_entity: Option<Entity>,
    pub render_target: Option<Handle<Image>>,
    pub gizmo_mode: GizmoMode,
    pub orbit_target: Vec3,
    pub orbit_distance: f32,
    pub orbit_angles: Vec2, // (yaw, pitch)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoMode {
    None,
    Translate,
    Rotate,
    Scale,
}

impl Default for SceneView {
    fn default() -> Self {
        Self {
            camera_entity: None,
            render_target: None,
            gizmo_mode: GizmoMode::None,
            orbit_target: Vec3::ZERO,
            orbit_distance: 10.0,
            orbit_angles: Vec2::new(0.0, std::f32::consts::PI / 4.0),
        }
    }
}
