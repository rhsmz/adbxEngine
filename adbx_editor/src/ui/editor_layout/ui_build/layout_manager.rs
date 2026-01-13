use bevy::prelude::*;
use super::super::EditorLayout;
use super::menu_bar::build_menu_bar;
use super::toolbar::build_toolbar;
use super::panel_container::build_panel_containers;

/// エディタのメインUIレイアウトを構築
pub fn build_editor_ui(
    mut commands: Commands,
    layout: Res<EditorLayout>,
) {
    // メインコンテナ
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Name::new("EditorRoot"),
    )).with_children(|parent| {
        // メニューバー
        build_menu_bar(parent);
        
        // ツールバー
        build_toolbar(parent);
        
        // パネルコンテナ
        build_panel_containers(parent, &layout);
    });
}
