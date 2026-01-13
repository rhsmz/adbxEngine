use super::super::resource::SearchReplace;
use bevy::prelude::*;

/// ダイアログのタイトル描画
pub fn draw_dialog_title(parent: &mut ChildSpawnerCommands, search_replace: &SearchReplace) {
    parent.spawn((
        Text::new(if search_replace.is_replace_visible {
            "置換"
        } else {
            "検索"
        }),
        bevy::text::TextFont {
            font_size: 14.0,
            ..default()
        },
        bevy::text::TextColor(Color::WHITE),
        Name::new("SearchReplaceTitle"),
    ));
}
