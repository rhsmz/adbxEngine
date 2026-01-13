use super::super::ErrorDialog;
use bevy::prelude::*;

/// エラーダイアログのタイトルバーを描画
pub fn draw_error_dialog_title_bar(
    parent: &mut ChildSpawnerCommands,
    error_dialog: &ErrorDialog,
    title_color: Color,
    icon: &str,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            Name::new("ErrorDialogTitleBar"),
        ))
        .with_children(|title_bar: &mut ChildSpawnerCommands| {
            // アイコンとタイトル
            title_bar
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Name::new("ErrorDialogTitle"),
                ))
                .with_children(|title: &mut ChildSpawnerCommands| {
                    title.spawn((
                        Text::new(icon),
                        bevy::text::TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        bevy::text::TextColor(title_color),
                        Name::new("ErrorDialogIcon"),
                    ));

                    title.spawn((
                        Text::new(&error_dialog.title),
                        bevy::text::TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                        Name::new("ErrorDialogTitleText"),
                    ));
                });

            // 閉じるボタン
            title_bar
                .spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    bevy::ui::Interaction::default(),
                    Name::new("ErrorDialogCloseButton"),
                ))
                .with_children(|close_button: &mut ChildSpawnerCommands| {
                    close_button.spawn((
                        Text::new("×"),
                        bevy::text::TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                        Name::new("ErrorDialogCloseButtonText"),
                    ));
                });
        });
}
