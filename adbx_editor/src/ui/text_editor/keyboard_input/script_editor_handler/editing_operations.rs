use crate::ui::script_editor::ScriptEditor;

/// バックスペース処理
pub fn handle_backspace_script_editor(script_editor: &mut ScriptEditor) {
    let cursor_pos = script_editor.cursor_position;
    if cursor_pos > 0 {
        let content = script_editor.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        if cursor_pos <= chars.len() && cursor_pos > 0 {
            // 履歴に保存
            let history_idx = script_editor.history_index;
            if script_editor.edit_history.len() > history_idx {
                script_editor.edit_history.truncate(history_idx);
            }
            script_editor.edit_history.push(content.clone());
            script_editor.history_index = script_editor.edit_history.len();
            
            chars.remove(cursor_pos - 1);
            script_editor.content = chars.into_iter().collect();
            script_editor.cursor_position = cursor_pos - 1;
            script_editor.content_entity = None;
        }
    }
}

/// 削除処理
pub fn handle_delete_script_editor(script_editor: &mut ScriptEditor) {
    let content = script_editor.content.clone();
    let mut chars: Vec<char> = content.chars().collect();
    if script_editor.cursor_position < chars.len() {
        // 履歴に保存
        let history_index = script_editor.history_index;
        if script_editor.edit_history.len() > history_index {
            script_editor.edit_history.truncate(history_index);
        }
        script_editor.edit_history.push(content.clone());
        script_editor.history_index = script_editor.edit_history.len();
        
        chars.remove(script_editor.cursor_position);
        script_editor.content = chars.into_iter().collect();
        script_editor.content_entity = None;
    }
}

/// Undo操作
pub fn undo_edit(script_editor: &mut ScriptEditor) {
    if script_editor.history_index > 0 {
        script_editor.history_index -= 1;
        if let Some(previous_content) = script_editor.edit_history.get(script_editor.history_index) {
            script_editor.content = previous_content.clone();
            script_editor.content_entity = None;
        }
    }
}

/// Redo操作
pub fn redo_edit(script_editor: &mut ScriptEditor) {
    if script_editor.history_index < script_editor.edit_history.len() {
        script_editor.history_index += 1;
        if let Some(next_content) = script_editor.edit_history.get(script_editor.history_index - 1) {
            script_editor.content = next_content.clone();
            script_editor.content_entity = None;
        }
    }
}
