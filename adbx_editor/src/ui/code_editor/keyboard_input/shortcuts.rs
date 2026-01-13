use super::super::resource::CodeEditor;
use super::super::completion::trigger_code_completion_for_current_position;
use super::super::file_operations::save_active_file_in_code_editor;
use super::editing_operations::{undo_edit, redo_edit};
use crate::ui::error_dialog::ErrorDialog;

/// ショートカットキーの処理
pub fn handle_shortcuts(
    code_editor: &mut CodeEditor,
    error_dialog: &mut ErrorDialog,
    keys: &bevy::prelude::Res<bevy::prelude::ButtonInput<bevy::prelude::KeyCode>>,
) {
    if keys.pressed(bevy::prelude::KeyCode::ControlLeft) || keys.pressed(bevy::prelude::KeyCode::ControlRight) {
        if keys.just_pressed(bevy::prelude::KeyCode::KeyA) {
            handle_select_all(code_editor);
        } else if keys.just_pressed(bevy::prelude::KeyCode::KeyS) {
            save_active_file_in_code_editor(code_editor, error_dialog);
        } else if keys.just_pressed(bevy::prelude::KeyCode::KeyZ) {
            undo_edit(code_editor);
        } else if keys.just_pressed(bevy::prelude::KeyCode::KeyY) || 
                  (keys.pressed(bevy::prelude::KeyCode::ShiftLeft) && keys.just_pressed(bevy::prelude::KeyCode::KeyZ)) {
            redo_edit(code_editor);
        } else if keys.just_pressed(bevy::prelude::KeyCode::Space) {
            trigger_code_completion_for_current_position(code_editor);
            code_editor.content_entity = None;
        }
    }
}

/// 全選択処理
fn handle_select_all(code_editor: &mut CodeEditor) {
    let active_tab = code_editor.active_tab;
    let current_content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    let content_len = current_content.len();
    drop(current_content);
    
    code_editor.selection_start = Some(0);
    code_editor.cursor_position = content_len;
    code_editor.content_entity = None;
}
