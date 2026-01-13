use crate::ui::script_editor::ScriptEditor;

/// 左矢印処理
pub fn handle_arrow_left_script_editor(script_editor: &mut ScriptEditor) {
    if script_editor.cursor_position > 0 {
        script_editor.cursor_position -= 1;
        script_editor.content_entity = None;
    }
}

/// 右矢印処理
pub fn handle_arrow_right_script_editor(script_editor: &mut ScriptEditor) {
    let max_pos = script_editor.content.chars().count();
    if script_editor.cursor_position < max_pos {
        script_editor.cursor_position += 1;
        script_editor.content_entity = None;
    }
}
