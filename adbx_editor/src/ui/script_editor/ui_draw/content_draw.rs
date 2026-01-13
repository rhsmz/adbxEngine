use bevy::prelude::*;
use bevy::ui::PositionType;
use super::super::{ScriptEditor, ScriptError};
use super::super::syntax_highlight::highlight_lua_text;
use super::super::cursor_position::get_cursor_position;

/// スクリプトコンテンツの描画
pub fn draw_script_content(
    commands: &mut Commands,
    text_entity: Entity,
    script_editor: &ScriptEditor,
    content_text: &str,
) {
    let is_focused = script_editor.is_focused;
    let cursor_position = script_editor.cursor_position;
    let errors = script_editor.errors.clone();
    
    commands.entity(text_entity).with_children(|text_node: &mut ChildSpawnerCommands| {
        draw_script_lines(text_node, content_text, &errors);
        draw_cursor(text_node, content_text, cursor_position, is_focused);
        draw_error_messages(text_node, &errors);
    });
}

/// スクリプト行の描画
fn draw_script_lines(
    parent: &mut ChildSpawnerCommands,
    content_text: &str,
    errors: &[ScriptError],
) {
    let lines: Vec<&str> = content_text.lines().collect();
    let highlighted_lines_vec = highlight_lua_text(content_text);
    let highlighted_lines: std::collections::HashMap<usize, Vec<(String, Color)>> = highlighted_lines_vec
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.clone()))
        .collect();
    
    for (i, line) in lines.iter().enumerate() {
        let has_error = errors.iter().any(|e| e.line == i + 1);
        
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            Name::new(format!("ScriptLine_{}", i)),
        )).with_children(|line_node: &mut ChildSpawnerCommands| {
            draw_line_number(line_node, i, has_error);
            draw_line_content(line_node, i, line, &highlighted_lines, has_error);
        });
    }
}

/// 行番号の描画
fn draw_line_number(parent: &mut ChildSpawnerCommands, line_index: usize, has_error: bool) {
    parent.spawn((
        Node {
            width: Val::Px(40.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            padding: UiRect::right(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(if has_error { Color::srgb(0.4, 0.1, 0.1) } else { Color::NONE }),
        Name::new(format!("LineNumber_{}", line_index)),
    )).with_children(|line_num: &mut ChildSpawnerCommands| {
        line_num.spawn((
            Text::new(&format!("{:4}", line_index + 1)),
            bevy::text::TextFont {
                font_size: 10.0,
                ..default()
            },
            bevy::text::TextColor(if has_error { Color::srgb(1.0, 0.5, 0.5) } else { Color::srgb(0.5, 0.5, 0.5) }),
        ));
    });
}

/// 行の内容の描画
fn draw_line_content(
    parent: &mut ChildSpawnerCommands,
    line_index: usize,
    line: &str,
    highlighted_lines: &std::collections::HashMap<usize, Vec<(String, Color)>>,
    has_error: bool,
) {
    parent.spawn((
        Node {
            flex_grow: 1.0,
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(if has_error { Color::srgb(0.3, 0.1, 0.1) } else { Color::NONE }),
        Name::new(format!("LineContent_{}", line_index)),
    )).with_children(|line_content: &mut ChildSpawnerCommands| {
        if let Some(highlighted_tokens) = highlighted_lines.get(&line_index) {
            for (token_text, token_color) in highlighted_tokens {
                if !token_text.is_empty() {
                    line_content.spawn((
                        Text::new(token_text),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(*token_color),
                        Name::new(format!("Token_{}", line_index)),
                    ));
                }
            }
        } else {
            line_content.spawn((
                Text::new(line),
                bevy::text::TextFont {
                    font_size: 11.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Name::new(format!("PlainText_{}", line_index)),
            ));
        }
    });
}

/// カーソルの描画
fn draw_cursor(
    parent: &mut ChildSpawnerCommands,
    content_text: &str,
    cursor_position: usize,
    is_focused: bool,
) {
    if is_focused {
        let (cursor_line, cursor_col) = get_cursor_position(content_text, cursor_position);
        let lines: Vec<&str> = content_text.lines().collect();
        
        if cursor_line < lines.len() {
            parent.spawn((
                Node {
                    width: Val::Px(2.0),
                    height: Val::Px(18.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(50.0 + (cursor_col as f32 * 6.6)), // 簡易的な文字幅計算
                    top: Val::Px(10.0 + (cursor_line as f32 * 22.0)),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                Name::new("ScriptEditorCursor"),
            ));
        }
    }
}

/// エラーメッセージの描画
fn draw_error_messages(parent: &mut ChildSpawnerCommands, errors: &[ScriptError]) {
    if !errors.is_empty() {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.1, 0.1)),
            Name::new("ScriptEditorErrors"),
        )).with_children(|error_area: &mut ChildSpawnerCommands| {
            error_area.spawn((
                Text::new("Errors:"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
            
            for error in errors {
                error_area.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(2.0)),
                        ..default()
                    },
                    Name::new(format!("ErrorLine_{}", error.line)),
                )).with_children(|error_line: &mut ChildSpawnerCommands| {
                    error_line.spawn((
                        Text::new(&format!("Line {}: {}", error.line, error.message)),
                        bevy::text::TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(1.0, 0.5, 0.5)),
                    ));
                });
            }
        });
    }
}
