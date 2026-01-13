use bevy::prelude::*;
use std::path::PathBuf;

/// スクリプトプレビューの描画
pub fn draw_script_preview(parent: &mut ChildSpawnerCommands, asset_file_path: &PathBuf) {
    if let Ok(content) = std::fs::read_to_string(asset_file_path) {
        let preview_lines: Vec<&str> = content.lines().take(20).collect();
        let preview_text = if content.lines().count() > 20 {
            format!(
                "{}\n... ({} lines total)",
                preview_lines.join("\n"),
                content.lines().count()
            )
        } else {
            preview_lines.join("\n")
        };

        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                Name::new("AssetBrowserScriptPreview"),
            ))
            .with_children(|script_preview: &mut ChildSpawnerCommands| {
                script_preview.spawn((
                    Text::new(&preview_text),
                    bevy::text::TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
    }
}

/// テキストプレビューの描画
pub fn draw_text_preview(parent: &mut ChildSpawnerCommands, asset_file_path: &PathBuf) {
    if let Ok(content) = std::fs::read_to_string(asset_file_path) {
        let preview_lines: Vec<&str> = content.lines().take(30).collect();
        let preview_text = if content.lines().count() > 30 {
            format!(
                "{}\n... ({} lines total)",
                preview_lines.join("\n"),
                content.lines().count()
            )
        } else {
            preview_lines.join("\n")
        };

        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                Name::new("AssetBrowserTextPreview"),
            ))
            .with_children(|text_preview: &mut ChildSpawnerCommands| {
                text_preview.spawn((
                    Text::new(&preview_text),
                    bevy::text::TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
    }
}
