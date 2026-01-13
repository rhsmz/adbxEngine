use super::super::resource::SearchReplace;
use bevy::prelude::*;

/// 検索オプションの描画
pub fn draw_search_options(parent: &mut ChildSpawnerCommands, search_replace: &SearchReplace) {
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
            Name::new("SearchOptionsRow"),
        ))
        .with_children(|row: &mut ChildSpawnerCommands| {
            // 大文字小文字を区別
            row.spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    margin: UiRect::right(Val::Px(5.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(if search_replace.case_sensitive {
                    Color::srgb(0.3, 0.5, 0.8)
                } else {
                    Color::srgb(0.2, 0.2, 0.2)
                }),
                bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                bevy::ui::Interaction::default(),
                Name::new("CaseSensitiveCheckbox"),
            ));

            row.spawn((
                Text::new("大文字小文字を区別"),
                bevy::text::TextFont {
                    font_size: 11.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("CaseSensitiveLabel"),
            ));
        });
}
