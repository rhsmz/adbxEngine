use super::super::resource::{EditorSettings, EditorTheme};
use bevy::prelude::*;

/// エディタ設定セクションの描画
pub fn draw_editor_settings_section(
    parent: &mut ChildSpawnerCommands,
    editor_settings: &EditorSettings,
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
            Name::new("EditorSettingsSection"),
        ))
        .with_children(|section: &mut ChildSpawnerCommands| {
            section.spawn((
                Text::new("Editor Settings"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            // テーマ設定
            draw_theme_setting(section, editor_settings);
        });
}

/// テーマ設定の描画
fn draw_theme_setting(parent: &mut ChildSpawnerCommands, editor_settings: &EditorSettings) {
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
            Name::new("ThemeSetting"),
        ))
        .with_children(|theme_row: &mut ChildSpawnerCommands| {
            theme_row.spawn((
                Text::new("Theme"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));

            let theme_text = match editor_settings.theme {
                EditorTheme::Dark => "Dark",
                EditorTheme::Light => "Light",
            };

            theme_row.spawn((
                Text::new(theme_text),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}
