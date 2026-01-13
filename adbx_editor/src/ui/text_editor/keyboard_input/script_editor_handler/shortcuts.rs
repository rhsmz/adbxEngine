use super::character_input::insert_text_at_cursor_script;
use crate::ui::script_editor::ScriptEditor;
use crate::ui::text_editor::TextEditorState;

/// 選択範囲をコピー
pub fn copy_selection(script_editor: &mut ScriptEditor, text_editor_state: &mut TextEditorState) {
    if let Some(start) = script_editor.selection_start {
        let start_pos = start.min(script_editor.cursor_position);
        let end_pos = start.max(script_editor.cursor_position);

        let chars: Vec<char> = script_editor.content.chars().collect();
        if start_pos < chars.len() && end_pos <= chars.len() {
            let selected: String = chars[start_pos..end_pos].iter().collect();
            text_editor_state.clipboard = selected;

            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(&text_editor_state.clipboard);
                }
            }
        }
    }
}

/// テキストをペースト
pub fn paste_text(script_editor: &mut ScriptEditor, text_editor_state: &mut TextEditorState) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if let Ok(clipboard_text) = clipboard.get_text() {
                text_editor_state.clipboard = clipboard_text;
            }
        }
    }

    if !text_editor_state.clipboard.is_empty() {
        insert_text_at_cursor_script(script_editor, text_editor_state.clipboard.clone());
    }
}

/// 選択範囲をカット
pub fn cut_selection(script_editor: &mut ScriptEditor, text_editor_state: &mut TextEditorState) {
    copy_selection(script_editor, text_editor_state);

    if let Some(start) = script_editor.selection_start {
        let start_pos = start.min(script_editor.cursor_position);
        let end_pos = start.max(script_editor.cursor_position);

        let content = script_editor.content.clone();
        let mut chars: Vec<char> = content.chars().collect();
        if start_pos < chars.len() && end_pos <= chars.len() {
            // 履歴に保存
            if script_editor.edit_history.len() > script_editor.history_index {
                script_editor
                    .edit_history
                    .truncate(script_editor.history_index);
            }
            script_editor.edit_history.push(content.clone());
            script_editor.history_index = script_editor.edit_history.len();

            chars.drain(start_pos..end_pos);
            script_editor.content = chars.into_iter().collect();
            script_editor.cursor_position = start_pos;
            script_editor.selection_start = None;
            script_editor.content_entity = None;
        }
    }
}

/// 全選択
pub fn select_all(script_editor: &mut ScriptEditor) {
    let content_len = script_editor.content.chars().count();
    script_editor.selection_start = Some(0);
    script_editor.cursor_position = content_len;
    script_editor.content_entity = None;
}
