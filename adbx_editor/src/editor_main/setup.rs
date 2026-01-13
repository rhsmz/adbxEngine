use bevy::prelude::*;

/// エディタの初期セットアップ
pub fn setup_editor(mut commands: Commands) {
    // エディタの初期セットアップ
    commands.spawn((bevy::camera::Camera2d::default(), Transform::default()));

    // テスト用のエンティティを追加（選択とGizmoのテスト用）
    commands.spawn((Name::new("TestEntity1"), Transform::from_xyz(0.0, 0.0, 0.0)));

    commands.spawn((Name::new("TestEntity2"), Transform::from_xyz(2.0, 0.0, 0.0)));

    commands.spawn((
        Name::new("TestEntity3"),
        Transform::from_xyz(-2.0, 0.0, 0.0),
    ));
}
