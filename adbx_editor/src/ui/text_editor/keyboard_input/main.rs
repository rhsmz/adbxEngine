use super::code_editor_handler::handle_code_editor_keyboard_input;
use super::script_editor_handler::handle_script_editor_keyboard_input;
use crate::ui::text_editor::TextEditorState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

/// テキストエディタのキーボード入力処理
pub fn handle_text_editor_input(
    mut code_editor: ResMut<crate::ui::code_editor::CodeEditor>,
    mut script_editor: ResMut<crate::ui::script_editor::ScriptEditor>,
    mut text_editor_state: ResMut<TextEditorState>,
    mut keyboard_input: MessageReader<KeyboardInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // コードエディタの入力処理
    handle_code_editor_keyboard_input(
        &mut code_editor,
        &mut text_editor_state,
        &mut keyboard_input,
        &keyboard,
    );

    // スクリプトエディタの入力処理
    handle_script_editor_keyboard_input(
        &mut script_editor,
        &mut text_editor_state,
        &mut keyboard_input,
        &keyboard,
    );
}
