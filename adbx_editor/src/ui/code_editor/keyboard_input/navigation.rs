use super::super::resource::CodeEditor;
use super::super::cursor_position::{calculate_cursor_line_and_column, calculate_char_position_from_line_column, get_line_length};

/// 左矢印キー処理
pub fn handle_arrow_left(code_editor: &mut CodeEditor) {
    if code_editor.cursor_position > 0 {
        code_editor.cursor_position -= 1;
        code_editor.content_entity = None;
    }
}

/// 右矢印キー処理
pub fn handle_arrow_right(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let content_len = current_content.len();
    drop(current_content);
    
    if code_editor.cursor_position < content_len {
        code_editor.cursor_position += 1;
        code_editor.content_entity = None;
    }
}

/// 上矢印キー処理
pub fn handle_arrow_up(code_editor: &mut CodeEditor) {
    // 補完が表示されている場合は補完候補を選択
    if code_editor.completion_state.is_visible {
        if code_editor.completion_state.selected_index > 0 {
            code_editor.completion_state.selected_index -= 1;
        }
        code_editor.content_entity = None;
    } else {
        let active_tab = code_editor.active_tab;
        let cursor_pos = code_editor.cursor_position;
        let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
            &active_file.content
        } else {
            &code_editor.content
        };
        
        let (current_line, current_col) = calculate_cursor_line_and_column(current_content, cursor_pos);
        if current_line > 0 {
            let new_line = current_line - 1;
            let new_col = current_col.min(get_line_length(current_content, new_line));
            let new_pos = calculate_char_position_from_line_column(current_content, new_line, new_col);
            drop(current_content);
            
            code_editor.cursor_position = new_pos;
            code_editor.content_entity = None;
        } else {
            drop(current_content);
        }
    }
}

/// 下矢印キー処理
pub fn handle_arrow_down(code_editor: &mut CodeEditor) {
    // 補完が表示されている場合は補完候補を選択
    if code_editor.completion_state.is_visible {
        if code_editor.completion_state.selected_index < code_editor.completion_state.candidates.len().saturating_sub(1) {
            code_editor.completion_state.selected_index += 1;
        }
        code_editor.content_entity = None;
    } else {
        let active_tab = code_editor.active_tab;
        let cursor_pos = code_editor.cursor_position;
        let scroll_offset = code_editor.scroll_offset;
        let visible_lines = code_editor.visible_lines;
        let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
            &active_file.content
        } else {
            &code_editor.content
        };
        
        let (current_line, current_col) = calculate_cursor_line_and_column(current_content, cursor_pos);
        let total_lines = current_content.lines().count();
        if current_line < total_lines - 1 {
            let new_line = current_line + 1;
            let new_col = current_col.min(get_line_length(current_content, new_line));
            let new_pos = calculate_char_position_from_line_column(current_content, new_line, new_col);
            
            // カーソルが表示範囲外になったらスクロール
            let visible_end_line = scroll_offset as usize + visible_lines;
            let new_scroll_offset = if new_line >= visible_end_line {
                (new_line as f32 - visible_lines as f32 + 1.0).max(0.0)
            } else {
                scroll_offset
            };
            
            drop(current_content);
            
            code_editor.cursor_position = new_pos;
            code_editor.scroll_offset = new_scroll_offset;
            code_editor.content_entity = None;
        } else {
            drop(current_content);
        }
    }
}

/// Homeキー処理
pub fn handle_home(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let (current_line, _) = calculate_cursor_line_and_column(current_content, cursor_pos);
    let new_pos = calculate_char_position_from_line_column(current_content, current_line, 0);
    drop(current_content);
    
    code_editor.cursor_position = new_pos;
    code_editor.content_entity = None;
}

/// Endキー処理
pub fn handle_end(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let (current_line, _) = calculate_cursor_line_and_column(current_content, cursor_pos);
    let line_length = get_line_length(current_content, current_line);
    let new_pos = calculate_char_position_from_line_column(current_content, current_line, line_length);
    drop(current_content);
    
    code_editor.cursor_position = new_pos;
    code_editor.content_entity = None;
}
