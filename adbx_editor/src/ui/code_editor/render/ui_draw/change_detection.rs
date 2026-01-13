use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use super::super::super::resource::CodeEditor;

/// 更新が必要かチェック
pub fn check_if_update_needed(code_editor: &mut CodeEditor) -> bool {
    let current_content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let mut hasher = DefaultHasher::new();
    current_content.hash(&mut hasher);
    let current_content_hash = hasher.finish();
    
    let content_changed = code_editor.last_content_hash != current_content_hash;
    let cursor_changed = code_editor.last_cursor_position != code_editor.cursor_position;
    let scroll_changed = (code_editor.last_scroll_offset - code_editor.scroll_offset).abs() > 0.1;
    let tab_changed = code_editor.last_active_tab != code_editor.active_tab;
    let completion_changed = code_editor.last_completion_visible != code_editor.completion_state.is_visible;
    
    code_editor.content_entity.is_none()
        || content_changed
        || cursor_changed
        || scroll_changed
        || tab_changed
        || completion_changed
}

/// 変更検知の状態を更新
pub fn update_change_detection(code_editor: &mut CodeEditor) {
    let current_content = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let mut hasher = DefaultHasher::new();
    current_content.hash(&mut hasher);
    let current_content_hash = hasher.finish();
    
    code_editor.last_content_hash = current_content_hash;
    code_editor.last_cursor_position = code_editor.cursor_position;
    code_editor.last_scroll_offset = code_editor.scroll_offset;
    code_editor.last_active_tab = code_editor.active_tab;
    code_editor.last_completion_visible = code_editor.completion_state.is_visible;
}
