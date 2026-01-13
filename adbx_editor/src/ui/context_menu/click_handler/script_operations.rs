use bevy::prelude::*;
use crate::ui::script_editor::ScriptEditor;

/// スクリプト操作のメニュー項目を処理
pub fn handle_script_operation_menu_item(
    script_editor: &mut ResMut<ScriptEditor>,
    operation: &str,
) {
    match operation {
        "ContextMenuRunScript" => {
            // スクリプト実行は別のシステムで処理
            script_editor.should_execute = true;
        }
        "ContextMenuValidateScript" => {
            // スクリプトを検証
            crate::ui::script_editor::validation::validate_script(script_editor.as_mut());
            bevy::log::info!("Validate Script");
        }
        _ => {}
    }
}
