use super::super::resource::SearchReplace;
use bevy::prelude::*;

/// 検索テキスト入力の描画
pub fn draw_search_input(parent: &mut ChildSpawnerCommands, search_replace: &SearchReplace) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Px(10.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("SearchInputRow"),
        ))
        .with_children(|row: &mut ChildSpawnerCommands| {
            row.spawn((
                Text::new("検索:"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("SearchLabel"),
            ));

            // 検索テキスト表示（簡易実装：実際の入力フィールドは後で実装）
            row.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Px(25.0),
                    margin: UiRect::left(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                Name::new("SearchTextInput"),
            ))
            .with_children(|input: &mut ChildSpawnerCommands| {
                input.spawn((
                    Text::new(if search_replace.search_text.is_empty() {
                        "検索文字列を入力..."
                    } else {
                        &search_replace.search_text
                    }),
                    bevy::text::TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    bevy::text::TextColor(if search_replace.search_text.is_empty() {
                        Color::srgb(0.5, 0.5, 0.5)
                    } else {
                        Color::WHITE
                    }),
                    Name::new("SearchTextDisplay"),
                ));
            });
        });
}
