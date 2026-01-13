use bevy::prelude::*;
use super::types::{ContextMenu, ContextType};
use super::menu_items::spawn_context_menu_items;

/// コンテキストメニューの表示
pub fn show_context_menu(
    mut commands: Commands,
    mut context_menu: ResMut<ContextMenu>,
    windows: Query<&Window>,
    context_type: ContextType,
) {
    // 既存のメニューを削除
    if let Some(menu_entity) = context_menu.menu_entity {
        if let Ok(mut entity_commands) = commands.get_entity(menu_entity) {
            entity_commands.despawn();
        }
    }
    
    // カーソル位置を取得
    let cursor_pos = if let Some(window) = windows.iter().next() {
        window.cursor_position().unwrap_or(Vec2::ZERO)
    } else {
        Vec2::ZERO
    };
    
    context_menu.is_visible = true;
    context_menu.position = cursor_pos;
    context_menu.context_type = Some(context_type);
    
    // メニューを表示
    let menu_entity = commands.spawn((
        Node {
            width: Val::Px(200.0),
            min_height: Val::Px(30.0),
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(cursor_pos.x),
            top: Val::Px(cursor_pos.y),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(2.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        Name::new("ContextMenu"),
    )).with_children(|menu: &mut ChildSpawnerCommands| {
        spawn_context_menu_items(menu, context_type);
    }).id();
    
    context_menu.menu_entity = Some(menu_entity);
}

/// コンテキストメニューを非表示にする
pub fn hide_context_menu(
    mut commands: Commands,
    mut context_menu: ResMut<ContextMenu>,
) {
    if let Some(menu_entity) = context_menu.menu_entity {
        if let Ok(mut entity_commands) = commands.get_entity(menu_entity) {
            entity_commands.despawn();
        }
    }
    context_menu.is_visible = false;
    context_menu.menu_entity = None;
    context_menu.context_type = None;
}
