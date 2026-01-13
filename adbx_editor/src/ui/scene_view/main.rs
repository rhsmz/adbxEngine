use bevy::prelude::*;
use super::resource::SceneView;

/// シーンビューのセットアップ
pub fn setup_scene_view(mut commands: Commands) {
    // 3Dカメラの作成
    let camera = commands.spawn((
        bevy::camera::Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("SceneViewCamera"),
    )).id();

    commands.insert_resource(SceneView {
        camera_entity: Some(camera),
        gizmo_mode: super::resource::GizmoMode::None,
        orbit_target: Vec3::ZERO,
        orbit_distance: 10.0,
        orbit_angles: Vec2::new(0.0, std::f32::consts::PI / 4.0),
    });
}
