use super::super::super::cursor_position::{
    calculate_cursor_line_and_column, calculate_cursor_x_pixel_position, get_line_number_width,
};
use bevy::prelude::*;

/// コード行の描画
pub fn draw_code_lines(
    parent: &mut ChildSpawnerCommands,
    content_text: &str,
    highlighted_lines: &std::collections::HashMap<usize, Vec<(String, Color)>>,
    scroll_offset: f32,
    visible_line_count: usize,
    line_height: f32,
) {
    let lines: Vec<&str> = content_text.lines().collect();
    let total_lines = lines.len();
    let start_line = scroll_offset as usize;
    let end_line = (start_line + visible_line_count + 1).min(total_lines);

    for (i, line) in lines.iter().enumerate() {
        if i < start_line || i >= end_line {
            continue;
        }

        let display_line_index = i - start_line;

        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(line_height),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::bottom(Val::Px(2.0)),
                    position_type: bevy::ui::PositionType::Absolute,
                    top: Val::Px(display_line_index as f32 * (line_height + 2.0)),
                    ..default()
                },
                Name::new(format!("CodeLine_{}", i)),
            ))
            .with_children(|line_node: &mut ChildSpawnerCommands| {
                // 行番号
                line_node
                    .spawn((
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::Center,
                            padding: UiRect::right(Val::Px(10.0)),
                            ..default()
                        },
                        Name::new(format!("CodeLineNumber_{}", i)),
                    ))
                    .with_children(|line_num: &mut ChildSpawnerCommands| {
                        line_num.spawn((
                            Text::new(&format!("{:4}", i + 1)),
                            bevy::text::TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            bevy::text::TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        ));
                    });

                // 行の内容（シンタックスハイライト付き）
                line_node
                    .spawn((
                        Node {
                            flex_grow: 1.0,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Name::new(format!("CodeLineContent_{}", i)),
                    ))
                    .with_children(|line_content: &mut ChildSpawnerCommands| {
                        if let Some(tokens) = highlighted_lines.get(&i) {
                            for (text, color) in tokens {
                                if !text.is_empty() {
                                    line_content.spawn((
                                        Text::new(text),
                                        bevy::text::TextFont {
                                            font_size: 11.0,
                                            ..default()
                                        },
                                        bevy::text::TextColor(*color),
                                    ));
                                }
                            }
                        } else {
                            line_content.spawn((
                                Text::new(*line),
                                bevy::text::TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                bevy::text::TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        }
                    });
            });
    }
}

/// カーソルの描画
pub fn draw_cursor(
    parent: &mut ChildSpawnerCommands,
    content_text: &str,
    cursor_pos: usize,
    is_focused: bool,
    scroll_offset: f32,
    visible_line_count: usize,
    line_height: f32,
) {
    if is_focused {
        let (cursor_line, cursor_col) = calculate_cursor_line_and_column(content_text, cursor_pos);
        let lines: Vec<&str> = content_text.lines().collect();
        let start_line = scroll_offset as usize;
        let end_line = (start_line + visible_line_count + 1).min(lines.len());

        if cursor_line >= start_line && cursor_line < end_line && cursor_line < lines.len() {
            let display_cursor_line = cursor_line - start_line;
            parent.spawn((
                Node {
                    width: Val::Px(2.0),
                    height: Val::Px(line_height - 2.0),
                    position_type: bevy::ui::PositionType::Absolute,
                    left: Val::Px(
                        get_line_number_width()
                            + calculate_cursor_x_pixel_position(
                                content_text,
                                cursor_line,
                                cursor_col,
                            ),
                    ),
                    top: Val::Px(display_cursor_line as f32 * (line_height + 2.0)),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                Name::new("CodeEditorCursor"),
            ));
        }
    }
}
