use crate::ui::code_editor::CodeEditor;

/// カーソル位置にテキストを挿入
pub fn insert_text_at_cursor(code_editor: &mut CodeEditor, text: String) {
    if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
        let content = active_file.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        let cursor_pos = code_editor.cursor_position.min(chars.len());
        
        // 履歴に保存
        if code_editor.edit_history.len() > code_editor.history_index {
            code_editor.edit_history.truncate(code_editor.history_index);
        }
        code_editor.edit_history.push(content.clone());
        code_editor.history_index = code_editor.edit_history.len();
        
        // テキストを挿入
        let insert_chars: Vec<char> = text.chars().collect();
        for (i, ch) in insert_chars.iter().enumerate() {
            chars.insert(cursor_pos + i, *ch);
        }
        active_file.content = chars.into_iter().collect();
        active_file.modified = true;
        code_editor.cursor_position += insert_chars.len();
        code_editor.content_entity = None;
    } else {
        let content = code_editor.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        let cursor_pos = code_editor.cursor_position.min(chars.len());
        
        // 履歴に保存
        if code_editor.edit_history.len() > code_editor.history_index {
            code_editor.edit_history.truncate(code_editor.history_index);
        }
        code_editor.edit_history.push(content.clone());
        code_editor.history_index = code_editor.edit_history.len();
        
        // テキストを挿入
        let insert_chars: Vec<char> = text.chars().collect();
        for (i, ch) in insert_chars.iter().enumerate() {
            chars.insert(cursor_pos + i, *ch);
        }
        code_editor.content = chars.into_iter().collect();
        code_editor.cursor_position += insert_chars.len();
        code_editor.content_entity = None;
    }
}
