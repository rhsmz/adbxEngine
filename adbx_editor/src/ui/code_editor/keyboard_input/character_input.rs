use super::super::completion::trigger_code_completion_for_current_position;
use super::super::resource::CodeEditor;
use super::editing_operations::insert_char_at_position;

/// 文字入力の処理
pub fn handle_character_input(code_editor: &mut CodeEditor, ch: &str) -> bool {
    // 制御文字は無視
    if ch.chars().next().map(|c| c.is_control()).unwrap_or(false) {
        return false;
    }

    let active_tab = code_editor.active_tab;
    let cursor_pos = code_editor.cursor_position;
    let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        &mut active_file.content
    } else {
        &mut code_editor.content
    };

    // 文字を挿入
    let ch_char = ch.chars().next().unwrap();
    let ch_len = ch.len();
    insert_char_at_position(current_content, cursor_pos, ch_char);
    let _ = current_content;

    code_editor.cursor_position = cursor_pos + ch_len;

    // 変更をマーク
    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        active_file.modified = true;
    }

    // 補完をトリガーする可能性のある文字を入力した場合
    if ch
        .chars()
        .next()
        .map(|c| c.is_alphanumeric() || c == '_')
        .unwrap_or(false)
    {
        trigger_code_completion_for_current_position(code_editor);
    } else {
        code_editor.completion_state.is_visible = false;
    }

    code_editor.content_entity = None;
    true
}
