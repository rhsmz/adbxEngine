use bevy::prelude::*;

/// パネルヘッダーの構築（共通）
pub fn build_panel_header(parent: &mut ChildSpawnerCommands, title: &str, panel_name: &str) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(25.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(5.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            Interaction::default(),
            super::super::super::super::docking::PanelHeader {
                panel_name: panel_name.to_string(),
            },
            Name::new(format!("{}Header", panel_name)),
        ))
        .with_children(|header| {
            // ドラッグハンドル（視覚的インジケーター）
            header
                .spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    Name::new("DragHandle"),
                ))
                .with_children(|handle| {
                    handle.spawn((
                        Text::new("⋮⋮"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });

            header.spawn((
                Text::new(title),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}
