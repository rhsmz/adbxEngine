use super::super::resource::CodeEditor;
use super::super::cursor_position::calculate_char_position_from_line_column;

/// 指定位置に文字を挿入
pub fn insert_char_at_position(text: &mut String, position: usize, ch: char) {
    text.insert(position, ch);
}

/// 指定位置の文字を削除
pub fn remove_char_at_position(text: &mut String, position: usize) {
    if position < text.len() {
        text.remove(position);
    }
}

/// バックスペース処理
pub fn handle_backspace(code_editor: &mut CodeEditor) {
    let cursor_pos = code_editor.cursor_position;
    if cursor_pos > 0 {
        let active_tab = code_editor.active_tab;
        let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
            &mut active_file.content
        } else {
            &mut code_editor.content
        };
        
        remove_char_at_position(current_content, cursor_pos - 1);
        drop(current_content);
        
        code_editor.cursor_position = cursor_pos - 1;

        if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
            active_file.modified = true;
        }

        code_editor.content_entity = None;
    }
}

/// 削除処理
pub fn handle_delete(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        &mut active_file.content
    } else {
        &mut code_editor.content
    };
    
    let content_len = current_content.len();
    if cursor_pos < content_len {
        remove_char_at_position(current_content, cursor_pos);
    }
    drop(current_content);

    if cursor_pos < content_len {
        if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
            active_file.modified = true;
        }
        code_editor.content_entity = None;
    }
}

/// Enterキー処理
pub fn handle_enter(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        &mut active_file.content
    } else {
        &mut code_editor.content
    };
    
    insert_char_at_position(current_content, cursor_pos, '\n');
    drop(current_content);
    
    code_editor.cursor_position = cursor_pos + 1;

    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        active_file.modified = true;
    }

    code_editor.content_entity = None;
}

/// Tabキー処理
pub fn handle_tab(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let mut cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        &mut active_file.content
    } else {
        &mut code_editor.content
    };
    
    // タブ文字を挿入（スペース4つ）
    for _ in 0..4 {
        insert_char_at_position(current_content, cursor_pos, ' ');
        cursor_pos += 1;
    }
    drop(current_content);
    
    code_editor.cursor_position = cursor_pos;

    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        active_file.modified = true;
    }

    code_editor.content_entity = None;
}

/// Undo操作
pub fn undo_edit(code_editor: &mut CodeEditor) {
    if code_editor.history_index > 0 {
        code_editor.history_index -= 1;
        if let Some(previous_content) = code_editor.edit_history.get(code_editor.history_index) {
            if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
                active_file.content = previous_content.clone();
                active_file.modified = true;
            } else {
                code_editor.content = previous_content.clone();
            }
            code_editor.content_entity = None;
        }
    }
}

/// Redo操作
pub fn redo_edit(code_editor: &mut CodeEditor) {
    if code_editor.history_index < code_editor.edit_history.len() {
        code_editor.history_index += 1;
        if let Some(next_content) = code_editor.edit_history.get(code_editor.history_index - 1) {
            if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
                active_file.content = next_content.clone();
                active_file.modified = true;
            } else {
                code_editor.content = next_content.clone();
            }
            code_editor.content_entity = None;
        }
    }
}
