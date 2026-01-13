use bevy::prelude::*;
use bevy::ui::Interaction;
use super::super::ScriptEditor;

/// スクリプトエディタのヘッダー描画
pub fn draw_script_editor_header(parent: &mut ChildSpawnerCommands, script_editor: &ScriptEditor) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("ScriptEditorHeader"),
    )).with_children(|header: &mut ChildSpawnerCommands| {
        // ファイル名表示
        draw_file_name(header, script_editor);
        
        // 保存ボタン
        draw_save_button(header);
    });
}

/// ファイル名の描画
fn draw_file_name(parent: &mut ChildSpawnerCommands, script_editor: &ScriptEditor) {
    parent.spawn((
        Node {
            flex_grow: 1.0,
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::left(Val::Px(5.0)),
            ..default()
        },
        Name::new("ScriptEditorFileName"),
    )).with_children(|file_name: &mut ChildSpawnerCommands| {
        let file_name_text = script_editor.current_script
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled Script");
        
        file_name.spawn((
            Text::new(file_name_text),
            bevy::text::TextFont {
                font_size: 12.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
    });
}

/// 保存ボタンの描画
fn draw_save_button(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Px(60.0),
            height: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::right(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
        Interaction::default(),
        Name::new("ScriptEditorSaveButton"),
    )).with_children(|button: &mut ChildSpawnerCommands| {
        button.spawn((
            Text::new("Save"),
            bevy::text::TextFont {
                font_size: 11.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
    });
}
