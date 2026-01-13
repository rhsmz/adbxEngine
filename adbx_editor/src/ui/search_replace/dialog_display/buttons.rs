use bevy::prelude::*;
use super::super::resource::SearchReplace;

/// ダイアログのボタン描画
pub fn draw_dialog_buttons(parent: &mut ChildSpawnerCommands, search_replace: &SearchReplace) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            flex_direction: FlexDirection::Row,
            margin: UiRect::top(Val::Px(10.0)),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            ..default()
        },
        Name::new("SearchReplaceButtons"),
    )).with_children(|buttons: &mut ChildSpawnerCommands| {
        // 検索ボタン
        buttons.spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.5, 0.8)),
            bevy::ui::Interaction::default(),
            Name::new("SearchButton"),
        )).with_children(|button: &mut ChildSpawnerCommands| {
            button.spawn((
                Text::new("検索"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // 置換ボタン（置換ダイアログの場合のみ）
        if search_replace.is_replace_visible {
            buttons.spawn((
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(25.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.3, 0.8)),
                bevy::ui::Interaction::default(),
                Name::new("ReplaceButton"),
            )).with_children(|button: &mut ChildSpawnerCommands| {
                button.spawn((
                    Text::new("置換"),
                    bevy::text::TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::WHITE),
                ));
            });
        }
        
        // 閉じるボタン
        buttons.spawn((
            Node {
                width: Val::Px(60.0),
                height: Val::Px(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            bevy::ui::Interaction::default(),
            Name::new("CloseButton"),
        )).with_children(|button: &mut ChildSpawnerCommands| {
            button.spawn((
                Text::new("閉じる"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}
