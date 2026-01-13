use crate::ui::script_editor::ScriptEditor;

/// カーソル位置にテキストを挿入
pub fn insert_text_at_cursor_script(script_editor: &mut ScriptEditor, text: String) {
    let content = script_editor.content.clone();
    let mut chars: Vec<char> = content.chars().collect();
    let cursor_pos = script_editor.cursor_position.min(chars.len());

    // 履歴に保存
    if script_editor.edit_history.len() > script_editor.history_index {
        script_editor
            .edit_history
            .truncate(script_editor.history_index);
    }
    script_editor.edit_history.push(content.clone());
    script_editor.history_index = script_editor.edit_history.len();

    // テキストを挿入
    let insert_chars: Vec<char> = text.chars().collect();
    for (i, ch) in insert_chars.iter().enumerate() {
        chars.insert(cursor_pos + i, *ch);
    }
    script_editor.content = chars.into_iter().collect();
    script_editor.cursor_position += insert_chars.len();
    script_editor.content_entity = None;
}
