use super::resource::SceneView;
use bevy::prelude::*;

/// シーンビューのUI描画
pub fn draw_scene_view_ui(
    parent: &mut ChildSpawnerCommands,
    _scene_view: &SceneView,
) {
    // ヘッダー
    parent.spawn((
        Text::new("Scene View"),
        bevy::text::TextFont {
            font_size: 16.0,
            ..default()
        },
        bevy::text::TextColor(Color::WHITE),
        Name::new("SceneViewHeader"),
    ));

    // プレースホルダー（Render-to-TextureはTODO）
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("SceneViewPlaceholder"),
        Text::new("Scene View\n(3D Camera Active - Render-to-Texture TODO)"),
    ));
}

/// シーンビューのセットアップ
pub fn setup_scene_view(mut commands: Commands) {
    // 3Dカメラの作成（UIカメラより低いorderで描画）
    let camera = commands
        .spawn((
            bevy::camera::Camera3d::default(),
            Camera {
                // order: -1 でUIカメラより先に描画（Camera order ambiguityを避ける）
                order: -1,
                ..default()
            },
            Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            Name::new("SceneViewCamera"),
        ))
        .id();

    commands.insert_resource(SceneView {
        camera_entity: Some(camera),
        render_target: None, // TODO: Render-to-Texture実装
        gizmo_mode: super::resource::GizmoMode::None,
        orbit_target: Vec3::ZERO,
        orbit_distance: 10.0,
        orbit_angles: Vec2::new(0.0, std::f32::consts::PI / 4.0),
    });
}
