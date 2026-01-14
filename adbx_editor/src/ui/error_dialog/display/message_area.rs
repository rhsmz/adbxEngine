use super::super::ErrorDialog;
use bevy::prelude::*;

/// エラーダイアログのメッセージエリアを描画
#[allow(dead_code)]
pub fn draw_error_dialog_message_area(
    parent: &mut ChildSpawnerCommands,
    error_dialog: &ErrorDialog,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            Name::new("ErrorDialogMessage"),
        ))
        .with_children(|message_area: &mut ChildSpawnerCommands| {
            // メッセージテキスト
            message_area.spawn((
                Text::new(&error_dialog.message),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("ErrorDialogMessageText"),
            ));

            // 詳細情報の展開/折りたたみボタン
            if error_dialog.details.is_some() || error_dialog.error.is_some() {
                draw_details_toggle(message_area, error_dialog);

                // 詳細情報の表示（展開されている場合のみ）
                if error_dialog.show_details {
                    draw_details_content(message_area, error_dialog);
                }
            }
        });
}

/// 詳細情報の展開/折りたたみボタンを描画
#[allow(dead_code)]
fn draw_details_toggle(parent: &mut ChildSpawnerCommands, error_dialog: &ErrorDialog) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            Name::new("ErrorDialogDetailsToggle"),
        ))
        .with_children(|toggle_area: &mut ChildSpawnerCommands| {
            // 詳細表示/非表示ボタン
            toggle_area
                .spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(25.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    bevy::ui::Interaction::default(),
                    Name::new("ErrorDialogDetailsToggleButton"),
                ))
                .with_children(|toggle_button: &mut ChildSpawnerCommands| {
                    toggle_button.spawn((
                        Text::new(if error_dialog.show_details {
                            "詳細を隠す"
                        } else {
                            "詳細を表示"
                        }),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                        Name::new("ErrorDialogDetailsToggleText"),
                    ));
                });

            // コピーボタン
            toggle_area
                .spawn((
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(25.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::left(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    bevy::ui::Interaction::default(),
                    Name::new("ErrorDialogCopyButton"),
                ))
                .with_children(|copy_button: &mut ChildSpawnerCommands| {
                    copy_button.spawn((
                        Text::new("コピー"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                        Name::new("ErrorDialogCopyButtonText"),
                    ));
                });
        });
}

/// 詳細情報の内容を描画
#[allow(dead_code)]
fn draw_details_content(parent: &mut ChildSpawnerCommands, error_dialog: &ErrorDialog) {
    let details_text = if let Some(ref error) = error_dialog.error {
        error.user_friendly_message()
    } else if let Some(ref details) = error_dialog.details {
        details.clone()
    } else {
        String::new()
    };

    if !details_text.is_empty() {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    max_height: Val::Px(200.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                Name::new("ErrorDialogDetails"),
            ))
            .with_children(|details_area: &mut ChildSpawnerCommands| {
                details_area.spawn((
                    Text::new(&details_text),
                    bevy::text::TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Name::new("ErrorDialogDetailsText"),
                ));
            });
    }
}
