use crate::ui::text_editor::TextEditorState;
use crate::ui::code_editor::CodeEditor;
use super::character_input::insert_text_at_cursor;

/// 選択範囲をコピー
pub fn copy_selection(
    code_editor: &mut CodeEditor,
    text_editor_state: &mut TextEditorState,
) {
    if let Some(start) = code_editor.selection_start {
        let start_pos = start.min(code_editor.cursor_position);
        let end_pos = start.max(code_editor.cursor_position);
        
        let content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
            &active_file.content
        } else {
            &code_editor.content
        };
        
        let chars: Vec<char> = content.chars().collect();
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
pub fn paste_text(
    code_editor: &mut CodeEditor,
    text_editor_state: &mut TextEditorState,
) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if let Ok(clipboard_text) = clipboard.get_text() {
                text_editor_state.clipboard = clipboard_text;
            }
        }
    }
    
    if !text_editor_state.clipboard.is_empty() {
        insert_text_at_cursor(code_editor, text_editor_state.clipboard.clone());
    }
}

/// 選択範囲をカット
pub fn cut_selection(
    code_editor: &mut CodeEditor,
    text_editor_state: &mut TextEditorState,
) {
    copy_selection(code_editor, text_editor_state);
    
    if let Some(start) = code_editor.selection_start {
        let start_pos = start.min(code_editor.cursor_position);
        let end_pos = start.max(code_editor.cursor_position);
        
        if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
            let content = active_file.content.clone();
            let mut chars: Vec<char> = content.chars().collect();
            if start_pos < chars.len() && end_pos <= chars.len() {
                if code_editor.edit_history.len() > code_editor.history_index {
                    code_editor.edit_history.truncate(code_editor.history_index);
                }
                code_editor.edit_history.push(content.clone());
                code_editor.history_index = code_editor.edit_history.len();
                
                chars.drain(start_pos..end_pos);
                active_file.content = chars.into_iter().collect();
                active_file.modified = true;
                code_editor.cursor_position = start_pos;
                code_editor.selection_start = None;
                code_editor.content_entity = None;
            }
        } else {
            let content = code_editor.content.clone();
            let mut chars: Vec<char> = content.chars().collect();
            if start_pos < chars.len() && end_pos <= chars.len() {
                if code_editor.edit_history.len() > code_editor.history_index {
                    code_editor.edit_history.truncate(code_editor.history_index);
                }
                code_editor.edit_history.push(content.clone());
                code_editor.history_index = code_editor.edit_history.len();
                
                chars.drain(start_pos..end_pos);
                code_editor.content = chars.into_iter().collect();
                code_editor.cursor_position = start_pos;
                code_editor.selection_start = None;
                code_editor.content_entity = None;
            }
        }
    }
}

/// 全選択
pub fn select_all(code_editor: &mut CodeEditor) {
    let content_len = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        active_file.content.chars().count()
    } else {
        code_editor.content.chars().count()
    };
    
    code_editor.selection_start = Some(0);
    code_editor.cursor_position = content_len;
    code_editor.content_entity = None;
}
