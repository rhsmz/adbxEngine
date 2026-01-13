use bevy::prelude::*;
use super::menu_items::{build_file_menu, build_edit_menu, build_build_menu, build_view_menu, build_help_menu};

/// メニューバーの構築
pub fn build_menu_bar(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        Name::new("MenuBar"),
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    )).with_children(|menu| {
        // File メニュー
        build_file_menu(menu);
        
        // Edit メニュー
        build_edit_menu(menu);
        
        // Build メニュー
        build_build_menu(menu);
        
        // View メニュー
        build_view_menu(menu);
        
        // Help メニュー
        build_help_menu(menu);
    });
}
