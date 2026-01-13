use bevy::prelude::*;

/// プロジェクト設定セクションのUI描画
pub fn draw_project_settings_section(
    parent: &mut ChildSpawnerCommands,
    project: &crate::project::Project,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            Name::new("ProjectSettingsSection"),
        ))
        .with_children(|section: &mut ChildSpawnerCommands| {
            section.spawn((
                Text::new("Project Settings"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            // プロジェクト名
            draw_project_name_setting(section, project);

            // シリアライゼーション形式
            draw_serialization_format_setting(section, project);
        });
}

/// プロジェクト名設定の描画
fn draw_project_name_setting(parent: &mut ChildSpawnerCommands, project: &crate::project::Project) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            Name::new("ProjectNameSetting"),
        ))
        .with_children(|name_row: &mut ChildSpawnerCommands| {
            name_row.spawn((
                Text::new("Project Name:"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            name_row.spawn((
                Text::new(&project.name),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}

/// シリアライゼーション形式設定の描画
fn draw_serialization_format_setting(
    parent: &mut ChildSpawnerCommands,
    project: &crate::project::Project,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            Name::new("SerializationFormatSetting"),
        ))
        .with_children(|format_row: &mut ChildSpawnerCommands| {
            format_row.spawn((
                Text::new("Serialization Format:"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            let format_text = match project.serialization_format {
                crate::project::SerializationFormat::Json => "JSON",
                crate::project::SerializationFormat::MessagePack => "MessagePack",
            };

            format_row.spawn((
                Text::new(format_text),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}

/// 設定パネルのフッター描画
pub fn draw_settings_panel_footer(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Name::new("SettingsPanelFooter"),
        ))
        .with_children(|footer: &mut ChildSpawnerCommands| {
            // 保存ボタン
            footer
                .spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                    bevy::ui::Interaction::default(),
                    Name::new("SettingsPanelSaveButton"),
                ))
                .with_children(|button: &mut ChildSpawnerCommands| {
                    button.spawn((
                        Text::new("Save"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });

            // キャンセルボタン
            footer
                .spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.3, 0.3)),
                    bevy::ui::Interaction::default(),
                    Name::new("SettingsPanelCancelButton"),
                ))
                .with_children(|button: &mut ChildSpawnerCommands| {
                    button.spawn((
                        Text::new("Cancel"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });
        });
}
