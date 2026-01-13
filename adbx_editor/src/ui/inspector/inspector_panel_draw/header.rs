use bevy::prelude::*;

/// エンティティ名ヘッダーの描画
pub fn draw_entity_header(parent: &mut ChildSpawnerCommands, entity_name: &str) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("EntityHeader"),
    )).with_children(|header| {
        header.spawn((
            Text::new(entity_name),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
    });
}

/// 何も選択されていない場合の表示
pub fn draw_no_selection(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("No entity selected"),
        bevy::text::TextFont {
            font_size: 14.0,
            ..default()
        },
        bevy::text::TextColor(Color::srgb(0.6, 0.6, 0.6)),
    ));
}
