use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use crate::ui::text_editor::TextEditorState;
use crate::ui::code_editor::CodeEditor;
use super::character_input::insert_text_at_cursor;
use super::editing_operations::{handle_backspace_code_editor, handle_delete_code_editor, undo_edit, redo_edit};
use super::navigation::{handle_arrow_left_code_editor, handle_arrow_right_code_editor, handle_arrow_up_code_editor, handle_arrow_down_code_editor, handle_home_code_editor, handle_end_code_editor};
use super::shortcuts::{copy_selection, paste_text, cut_selection, select_all};

/// コードエディタのキーボード入力処理
pub fn handle_code_editor_keyboard_input(
    mut code_editor: &mut CodeEditor,
    mut text_editor_state: &mut TextEditorState,
    keyboard_input: &mut MessageReader<KeyboardInput>,
    keyboard: &Res<ButtonInput<KeyCode>>,
) {
    if !code_editor.is_focused {
        return;
    }
    
    for event in keyboard_input.read() {
        if event.state.is_pressed() {
            match &event.logical_key {
                Key::Backspace => {
                    handle_backspace_code_editor(&mut code_editor);
                }
                Key::Delete => {
                    handle_delete_code_editor(&mut code_editor);
                }
                Key::ArrowLeft => {
                    handle_arrow_left_code_editor(&mut code_editor);
                }
                Key::ArrowRight => {
                    handle_arrow_right_code_editor(&mut code_editor);
                }
                Key::ArrowUp => {
                    handle_arrow_up_code_editor(&mut code_editor);
                }
                Key::ArrowDown => {
                    handle_arrow_down_code_editor(&mut code_editor);
                }
                Key::Home => {
                    handle_home_code_editor(&mut code_editor);
                }
                Key::End => {
                    handle_end_code_editor(&mut code_editor);
                }
                Key::Enter => {
                    insert_text_at_cursor(&mut code_editor, "\n".to_string());
                }
                Key::Tab => {
                    insert_text_at_cursor(&mut code_editor, "    ".to_string()); // 4スペース
                }
                Key::Character(ch) => {
                    insert_text_at_cursor(&mut code_editor, ch.to_string());
                }
                _ => {}
            }
        }
    }
    
    // ショートカットキーの処理
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        if keyboard.just_pressed(KeyCode::KeyC) {
            copy_selection(&mut code_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyV) {
            paste_text(&mut code_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyX) {
            cut_selection(&mut code_editor, &mut text_editor_state);
        } else if keyboard.just_pressed(KeyCode::KeyZ) {
            undo_edit(&mut code_editor);
        } else if keyboard.just_pressed(KeyCode::KeyY) || 
                  (keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::KeyZ)) {
            redo_edit(&mut code_editor);
        } else if keyboard.just_pressed(KeyCode::KeyA) {
            select_all(&mut code_editor);
        }
    }
}
