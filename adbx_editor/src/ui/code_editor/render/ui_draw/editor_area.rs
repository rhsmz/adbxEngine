use bevy::prelude::*;
use super::super::super::resource::CodeEditor;
use super::code_lines::{draw_code_lines, draw_cursor};

/// エディタエリアエンティティを作成
pub fn create_editor_area_entity(commands: &mut Commands) -> Entity {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(5.0)),
            overflow: Overflow::clip_y(),
            margin: UiRect::top(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        Name::new("CodeEditorArea"),
    )).id()
}

/// テキストエリアの描画
pub fn draw_text_area(parent: &mut ChildSpawnerCommands, code_editor: &CodeEditor) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        BackgroundColor({
            let is_focused = code_editor.is_focused;
            if is_focused { 
                Color::srgb(0.12, 0.12, 0.12) 
            } else { 
                Color::srgb(0.1, 0.1, 0.1) 
            }
        }),
        bevy::ui::Interaction::default(),
        Name::new("CodeEditorTextArea"),
    ));
}

/// テキストエリアの内容を描画
pub fn draw_text_area_content(
    commands: &mut Commands,
    code_editor: &mut CodeEditor,
    text_area_entity: Entity,
) {
    let active_tab = code_editor.active_tab;
    let content_text = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        active_file.content.clone()
    } else if code_editor.content.is_empty() {
        "// コードエディタ\n// ファイルを開くか、新規ファイルを作成してください".to_string()
    } else {
        code_editor.content.clone()
    };
    
    let viewport_height = 800.0;
    let line_height = code_editor.line_height;
    let visible_line_count = (viewport_height / line_height).ceil() as usize;
    code_editor.visible_lines = visible_line_count;
    
    let scroll_offset = code_editor.scroll_offset;
    let current_file_path = code_editor.open_files.get(active_tab)
        .map(|f| f.path.clone())
        .or_else(|| code_editor.current_file.clone());
    
    let language = current_file_path.as_ref()
        .and_then(|p| super::super::super::syntax_highlight::detect_programming_language_from_file_path(p));
    let highlighted_lines_vec = super::super::super::syntax_highlight::apply_syntax_highlighting_to_code(
        &content_text,
        language,
        code_editor,
    );
    let highlighted_lines: std::collections::HashMap<usize, Vec<(String, Color)>> = highlighted_lines_vec
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.clone()))
        .collect();
    
    let line_height = code_editor.line_height;
    let cursor_pos = code_editor.cursor_position;
    let is_focused = code_editor.is_focused;
    
    code_editor.text_area_entity = Some(text_area_entity);
    
    commands.entity(text_area_entity).with_children(|text_area_inner: &mut ChildSpawnerCommands| {
        draw_code_lines(text_area_inner, &content_text, &highlighted_lines, scroll_offset, visible_line_count, line_height);
        draw_cursor(text_area_inner, &content_text, cursor_pos, is_focused, scroll_offset, visible_line_count, line_height);
    });
}
