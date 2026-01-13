use bevy::prelude::*;
use super::super::resource::SearchReplace;

/// 置換テキスト入力の描画
pub fn draw_replace_input(parent: &mut ChildSpawnerCommands, search_replace: &SearchReplace) {
    if !search_replace.is_replace_visible {
        return;
    }
    
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            flex_direction: FlexDirection::Row,
            margin: UiRect::top(Val::Px(10.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        Name::new("ReplaceInputRow"),
    )).with_children(|row: &mut ChildSpawnerCommands| {
        row.spawn((
            Text::new("置換:"),
            bevy::text::TextFont {
                font_size: 12.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
            Name::new("ReplaceLabel"),
        ));
        
        // 置換テキスト表示（簡易実装：実際の入力フィールドは後で実装）
        row.spawn((
            Node {
                width: Val::Percent(70.0),
                height: Val::Px(25.0),
                margin: UiRect::left(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            Name::new("ReplaceTextInput"),
        )).with_children(|input: &mut ChildSpawnerCommands| {
            input.spawn((
                Text::new(if search_replace.replace_text.is_empty() {
                    "置換文字列を入力..."
                } else {
                    &search_replace.replace_text
                }),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(if search_replace.replace_text.is_empty() {
                    Color::srgb(0.5, 0.5, 0.5)
                } else {
                    Color::WHITE
                }),
                Name::new("ReplaceTextDisplay"),
            ));
        });
    });
}
