#[cfg(test)]
mod tests {
    use adbx_editor::ui::scene_view::{SceneView, GizmoMode};
    use bevy::prelude::*;

    #[test]
    fn test_scene_view_resource_creation() {
        let scene_view = SceneView::default();

        assert!(scene_view.camera_entity.is_none());
        assert!(scene_view.render_target.is_none());
        assert_eq!(scene_view.gizmo_mode, GizmoMode::None);
        assert_eq!(scene_view.orbit_target, Vec3::ZERO);
        assert_eq!(scene_view.orbit_distance, 10.0);
        assert_eq!(scene_view.orbit_angles, Vec2::new(0.0, std::f32::consts::PI / 4.0));
    }

    #[test]
    fn test_gizmo_mode_transitions() {
        let mut scene_view = SceneView::default();

        scene_view.gizmo_mode = GizmoMode::Translate;
        assert_eq!(scene_view.gizmo_mode, GizmoMode::Translate);

        scene_view.gizmo_mode = GizmoMode::Rotate;
        assert_eq!(scene_view.gizmo_mode, GizmoMode::Rotate);

        scene_view.gizmo_mode = GizmoMode::Scale;
        assert_eq!(scene_view.gizmo_mode, GizmoMode::Scale);

        scene_view.gizmo_mode = GizmoMode::None;
        assert_eq!(scene_view.gizmo_mode, GizmoMode::None);
    }

    #[test]
    fn test_scene_view_camera_properties() {
        let mut scene_view = SceneView::default();

        scene_view.orbit_target = Vec3::new(1.0, 2.0, 3.0);
        scene_view.orbit_distance = 15.0;
        scene_view.orbit_angles = Vec2::new(1.57, 0.78);

        assert_eq!(scene_view.orbit_target, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(scene_view.orbit_distance, 15.0);
        assert_eq!(scene_view.orbit_angles, Vec2::new(1.57, 0.78));
    }

    // 統合テスト: Bevy AppでのSceneViewセットアップ検証
    #[test]
    fn test_scene_view_setup_integration() {
        let mut app = App::new();

        // SceneViewリソースを初期化
        app.init_resource::<SceneView>();

        // setup_scene_viewシステムを実行
        app.add_systems(Startup, adbx_editor::ui::scene_view::setup_scene_view);

        // 1フレーム実行してシステムが動作することを確認
        app.update();

        // SceneViewリソースが初期化されていることを確認
        let scene_view = app.world().get_resource::<SceneView>().unwrap();
        assert_eq!(scene_view.gizmo_mode, GizmoMode::None);
        assert!(scene_view.render_target.is_none()); // まだ実装されていない

        // Cameraエンティティが作成されていることを確認
        if let Some(camera_entity) = scene_view.camera_entity {
            let camera = app.world().get::<Camera>(camera_entity);
            assert!(camera.is_some(), "Camera component should exist");

            let camera = camera.unwrap();
            assert_eq!(camera.order, -1, "Camera order should be -1 to avoid ambiguity");
        } else {
            panic!("Camera entity should be created");
        }
    }
}
