use bevy::prelude::*;

/// 空のプレビューの描画
pub fn draw_empty_preview(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("AssetBrowserPreviewEmpty"),
        ))
        .with_children(|empty: &mut ChildSpawnerCommands| {
            empty.spawn((
                Text::new("アセットを選択してください"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

/// プレビューエラーの描画
pub fn draw_preview_error(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("AssetBrowserPreviewError"),
        ))
        .with_children(|error: &mut ChildSpawnerCommands| {
            error.spawn((
                Text::new("選択されたアセットが見つかりません"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(0.8, 0.3, 0.3)),
            ));
        });
}
