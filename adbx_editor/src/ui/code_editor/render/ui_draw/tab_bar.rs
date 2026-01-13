use super::super::super::resource::CodeEditor;
use bevy::prelude::*;

/// タブバーの描画
pub fn draw_tab_bar(parent: &mut ChildSpawnerCommands, code_editor: &CodeEditor) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(2.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            Name::new("CodeEditorTabs"),
        ))
        .with_children(|tabs: &mut ChildSpawnerCommands| {
            // 開いているファイルのタブを表示
            for (i, file) in code_editor.open_files.iter().enumerate() {
                let is_active = i == code_editor.active_tab;
                tabs.spawn((
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(25.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(if is_active {
                        Color::srgb(0.3, 0.3, 0.3)
                    } else {
                        Color::srgb(0.25, 0.25, 0.25)
                    }),
                    bevy::ui::Interaction::default(),
                    Name::new(format!("CodeEditorTab_{}", i)),
                ))
                .with_children(|tab: &mut ChildSpawnerCommands| {
                    let file_name = std::path::Path::new(&file.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Untitled");

                    let tab_text = if file.modified {
                        format!("{} *", file_name)
                    } else {
                        file_name.to_string()
                    };

                    tab.spawn((
                        Text::new(&tab_text),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });
            }

            // 新規ファイルボタン
            tabs.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(25.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                bevy::ui::Interaction::default(),
                Name::new("CodeEditorNewFileButton"),
            ))
            .with_children(|button: &mut ChildSpawnerCommands| {
                button.spawn((
                    Text::new("+"),
                    bevy::text::TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::WHITE),
                ));
            });
        });
}
