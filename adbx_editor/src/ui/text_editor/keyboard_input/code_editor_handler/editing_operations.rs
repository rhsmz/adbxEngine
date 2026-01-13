use crate::ui::code_editor::CodeEditor;

/// バックスペース処理
pub fn handle_backspace_code_editor(code_editor: &mut CodeEditor) {
    let cursor_pos = code_editor.cursor_position;
    if cursor_pos > 0 {
        let active_tab = code_editor.active_tab;
        if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
            let content = active_file.content.clone();
            let mut chars: Vec<char> = content.chars().collect();
            if cursor_pos <= chars.len() {
                chars.remove(cursor_pos - 1);
                active_file.content = chars.into_iter().collect();
                active_file.modified = true;
                code_editor.cursor_position = cursor_pos - 1;
                code_editor.content_entity = None;
            }
        } else {
            let content = code_editor.content.clone();
            let mut chars: Vec<char> = content.chars().collect();
            if cursor_pos <= chars.len() && cursor_pos > 0 {
                chars.remove(cursor_pos - 1);
                code_editor.content = chars.into_iter().collect();
                code_editor.cursor_position = cursor_pos - 1;
                code_editor.content_entity = None;
            }
        }
    }
}

/// 削除処理
pub fn handle_delete_code_editor(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        let content = active_file.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        if cursor_pos < chars.len() {
            chars.remove(cursor_pos);
            active_file.content = chars.into_iter().collect();
            active_file.modified = true;
            code_editor.content_entity = None;
        }
    } else {
        let content = code_editor.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        if cursor_pos < chars.len() {
            chars.remove(cursor_pos);
            code_editor.content = chars.into_iter().collect();
            code_editor.content_entity = None;
        }
    }
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
