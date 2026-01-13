use super::super::navigation::{get_char_position, get_cursor_position, get_line_length};
use crate::ui::code_editor::CodeEditor;

/// 左矢印処理
pub fn handle_arrow_left_code_editor(code_editor: &mut CodeEditor) {
    if code_editor.cursor_position > 0 {
        code_editor.cursor_position -= 1;
        code_editor.content_entity = None;
    }
}

/// 右矢印処理
pub fn handle_arrow_right_code_editor(code_editor: &mut CodeEditor) {
    let max_pos = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        active_file.content.chars().count()
    } else {
        code_editor.content.chars().count()
    };
    if code_editor.cursor_position < max_pos {
        code_editor.cursor_position += 1;
        code_editor.content_entity = None;
    }
}

/// 上矢印処理
pub fn handle_arrow_up_code_editor(code_editor: &mut CodeEditor) {
    let content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    let (current_line, current_col) = get_cursor_position(content, code_editor.cursor_position);
    if current_line > 0 {
        let new_line = current_line - 1;
        let new_col = current_col.min(get_line_length(content, new_line));
        code_editor.cursor_position = get_char_position(content, new_line, new_col);
        code_editor.content_entity = None;
    }
}

/// 下矢印処理
pub fn handle_arrow_down_code_editor(code_editor: &mut CodeEditor) {
    let content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    let (current_line, current_col) = get_cursor_position(content, code_editor.cursor_position);
    let total_lines = content.lines().count();
    if current_line < total_lines.saturating_sub(1) {
        let new_line = current_line + 1;
        let new_col = current_col.min(get_line_length(content, new_line));
        code_editor.cursor_position = get_char_position(content, new_line, new_col);
        code_editor.content_entity = None;
    } else if current_line == total_lines.saturating_sub(1) {
        code_editor.cursor_position = content.len();
        code_editor.content_entity = None;
    }
}

/// Homeキー処理
pub fn handle_home_code_editor(code_editor: &mut CodeEditor) {
    let content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    let (current_line, _) = get_cursor_position(content, code_editor.cursor_position);
    code_editor.cursor_position = get_char_position(content, current_line, 0);
    code_editor.content_entity = None;
}

/// Endキー処理
pub fn handle_end_code_editor(code_editor: &mut CodeEditor) {
    let content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    let (current_line, _) = get_cursor_position(content, code_editor.cursor_position);
    let line_length = get_line_length(content, current_line);
    code_editor.cursor_position = get_char_position(content, current_line, line_length);
    code_editor.content_entity = None;
}
