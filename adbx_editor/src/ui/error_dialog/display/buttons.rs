use bevy::prelude::*;

/// エラーダイアログのボタンエリアを描画
pub fn draw_error_dialog_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Name::new("ErrorDialogButtons"),
    )).with_children(|buttons: &mut ChildSpawnerCommands| {
        buttons.spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            bevy::ui::Interaction::default(),
            Name::new("ErrorDialogOkButton"),
        )).with_children(|ok_button: &mut ChildSpawnerCommands| {
            ok_button.spawn((
                Text::new("OK"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("ErrorDialogOkButtonText"),
            ));
        });
    });
}
