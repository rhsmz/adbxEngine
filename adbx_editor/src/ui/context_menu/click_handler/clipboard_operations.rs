use super::super::{handle_entity_clipboard_operations, EntityClipboardOperation};
use crate::systems::selection::Selection;
use crate::ui::clipboard::Clipboard;
use crate::ui::code_editor::CodeEditor;
use crate::ui::context_menu::{
    handle_text_editor_clipboard_operations, TextEditorClipboardOperation,
};
use crate::ui::script_editor::ScriptEditor;
use crate::ui::text_editor::TextEditorState;
use bevy::prelude::*;

/// クリップボード操作のメニュー項目を処理
pub fn handle_clipboard_operation_menu_item(
    code_editor: &mut ResMut<CodeEditor>,
    script_editor: &mut ResMut<ScriptEditor>,
    text_editor_state: &mut ResMut<TextEditorState>,
    commands: &mut Commands,
    selection: &mut ResMut<Selection>,
    clipboard: &mut ResMut<Clipboard>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
    operation: TextEditorClipboardOperation,
) {
    if code_editor.is_focused || script_editor.is_focused {
        handle_text_editor_clipboard_operations(
            code_editor,
            script_editor,
            text_editor_state,
            operation,
        );
    } else {
        handle_entity_clipboard_operations(
            commands,
            selection,
            clipboard,
            transform_query,
            name_query,
            match operation {
                TextEditorClipboardOperation::Cut => EntityClipboardOperation::Cut,
                TextEditorClipboardOperation::Copy => EntityClipboardOperation::Copy,
                TextEditorClipboardOperation::Paste => EntityClipboardOperation::Paste,
            },
        );
    }
}
