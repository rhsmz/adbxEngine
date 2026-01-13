use crate::ui::code_editor::CodeEditor;
use bevy::prelude::*;

/// テキストエディタ操作のメニュー項目を処理
pub fn handle_text_editor_operation_menu_item(
    code_editor: &mut ResMut<CodeEditor>,
    operation: &str,
) {
    match operation {
        "ContextMenuSelectAll" => {
            // 全選択
            let active_tab = code_editor.active_tab;
            if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
                let content_len = active_file.content.len();
                code_editor.selection_start = Some(0);
                code_editor.cursor_position = content_len;
            }
        }
        _ => {}
    }
}
