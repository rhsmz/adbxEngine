use bevy::prelude::*;
use super::super::RenameDialogRequest;

/// リネームダイアログのUI描画
pub fn draw_rename_dialog(
    mut commands: Commands,
    rename_dialog: &RenameDialogRequest,
) {
    if !rename_dialog.is_visible {
        return;
    }
    
    if let Some(_target_path) = &rename_dialog.target_path {
        // ダイアログの背景（オーバーレイ）
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            Name::new("RenameDialogOverlay"),
        )).with_children(|overlay| {
            // ダイアログウィンドウ
            overlay.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(150.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                Name::new("RenameDialog"),
            )).with_children(|dialog| {
                draw_dialog_title(dialog);
                draw_current_name_display(dialog, rename_dialog);
                draw_input_field(dialog, rename_dialog);
                draw_dialog_buttons(dialog);
            });
        });
    }
}

/// ダイアログタイトルの描画
fn draw_dialog_title(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("アセットのリネーム"),
        bevy::text::TextFont {
            font_size: 18.0,
            ..default()
        },
        bevy::text::TextColor(Color::WHITE),
        Name::new("RenameDialogTitle"),
    ));
}

/// 現在の名前表示の描画
fn draw_current_name_display(parent: &mut ChildSpawnerCommands, rename_dialog: &RenameDialogRequest) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
        Name::new("RenameDialogCurrentName"),
    )).with_children(|current_name| {
        current_name.spawn((
            Text::new(format!("現在の名前: {}", rename_dialog.current_name)),
            bevy::text::TextFont {
                font_size: 12.0,
                ..default()
            },
            bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

/// 入力フィールドの描画
fn draw_input_field(parent: &mut ChildSpawnerCommands, rename_dialog: &RenameDialogRequest) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            margin: UiRect::top(Val::Px(10.0)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        Name::new("RenameDialogInput"),
    )).with_children(|input| {
        input.spawn((
            Text::new(&rename_dialog.new_name),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
            Name::new("RenameDialogInputText"),
        ));
    });
}

/// ダイアログボタンの描画
fn draw_dialog_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
        Name::new("RenameDialogButtons"),
    )).with_children(|buttons| {
        // キャンセルボタン
        buttons.spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            Interaction::default(),
            Name::new("RenameDialogCancel"),
        )).with_children(|cancel| {
            cancel.spawn((
                Text::new("キャンセル"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // OKボタン
        buttons.spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.5, 0.8)),
            Interaction::default(),
            Name::new("RenameDialogOK"),
        )).with_children(|ok| {
            ok.spawn((
                Text::new("OK"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}
