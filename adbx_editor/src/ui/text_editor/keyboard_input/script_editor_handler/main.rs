use super::character_input::insert_text_at_cursor_script;
use super::editing_operations::{
    handle_backspace_script_editor, handle_delete_script_editor, redo_edit, undo_edit,
};
use super::navigation::{handle_arrow_left_script_editor, handle_arrow_right_script_editor};
use super::shortcuts::{copy_selection, cut_selection, paste_text, select_all};
use crate::ui::script_editor::ScriptEditor;
use crate::ui::text_editor::TextEditorState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

/// スクリプトエディタのキーボード入力処理
pub fn handle_script_editor_keyboard_input(
    mut script_editor: &mut ScriptEditor,
    mut text_editor_state: &mut TextEditorState,
    keyboard_input: &mut MessageReader<KeyboardInput>,
    keyboard: &Res<ButtonInput<KeyCode>>,
) {
    if !script_editor.is_focused {
        return;
    }

    for event in keyboard_input.read() {
        if event.state.is_pressed() {
            match &event.logical_key {
                Key::Backspace => {
                    handle_backspace_script_editor(&mut script_editor);
                }
                Key::Delete => {
                    handle_delete_script_editor(&mut script_editor);
                }
                Key::ArrowLeft => {
                    handle_arrow_left_script_editor(&mut script_editor);
                }
                Key::ArrowRight => {
                    handle_arrow_right_script_editor(&mut script_editor);
                }
                Key::Enter => {
                    insert_text_at_cursor_script(&mut script_editor, "\n".to_string());
                }
                Key::Tab => {
                    insert_text_at_cursor_script(&mut script_editor, "    ".to_string());
                    // 4スペース
                }
                Key::Character(ch) => {
                    insert_text_at_cursor_script(&mut script_editor, ch.to_string());
                }
                _ => {}
            }
        }
    }

    // ショートカットキーの処理
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        if keyboard.just_pressed(KeyCode::KeyC) {
            copy_selection(&mut script_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyV) {
            paste_text(&mut script_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyX) {
            cut_selection(&mut script_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyZ) {
            undo_edit(&mut script_editor);
        } else if keyboard.just_pressed(KeyCode::KeyY)
            || (keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::KeyZ))
        {
            redo_edit(&mut script_editor);
        } else if keyboard.just_pressed(KeyCode::KeyA) {
            select_all(&mut script_editor);
        }
    }
}
