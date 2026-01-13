use bevy::prelude::*;

/// 設定パネルのヘッダー描画
pub fn draw_settings_panel_header(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Name::new("SettingsPanelHeader"),
        ))
        .with_children(|header: &mut ChildSpawnerCommands| {
            header.spawn((
                Text::new("Settings"),
                bevy::text::TextFont {
                    font_size: 18.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            // 閉じるボタン
            header
                .spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                    bevy::ui::Interaction::default(),
                    Name::new("SettingsPanelCloseButton"),
                ))
                .with_children(|button: &mut ChildSpawnerCommands| {
                    button.spawn((
                        Text::new("×"),
                        bevy::text::TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });
        });
}
