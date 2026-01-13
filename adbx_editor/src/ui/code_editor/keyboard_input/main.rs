use bevy::prelude::*;
use super::super::resource::CodeEditor;
use super::character_input::handle_character_input;
use super::editing_operations::{handle_backspace, handle_delete, handle_enter, handle_tab};
use super::navigation::{handle_arrow_left, handle_arrow_right, handle_arrow_up, handle_arrow_down, handle_home, handle_end};
use super::shortcuts::handle_shortcuts;
use super::completion_navigation::{handle_completion_navigation, handle_manual_completion_trigger};

/// キーボード入力の処理
pub fn handle_code_editor_keyboard_input(
    mut code_editor: ResMut<CodeEditor>,
    mut error_dialog: ResMut<crate::ui::error_dialog::ErrorDialog>,
    mut keyboard_input: MessageReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // エディタがフォーカスされていない場合は処理しない
    if !code_editor.is_focused {
        return;
    }

    let _active_tab = code_editor.active_tab;

    // キーボードイベントの処理
    for event in keyboard_input.read() {
        if !event.state.is_pressed() {
            continue;
        }

        // 文字入力の処理
        if let bevy::input::keyboard::Key::Character(ch) = &event.logical_key {
            if handle_character_input(&mut code_editor, ch) {
                continue;
            }
        }

        // 補完ナビゲーション
        if handle_completion_navigation(&mut code_editor, &event.logical_key) {
            continue;
        }
        
        // Ctrl+Spaceで補完を手動でトリガー
        if handle_manual_completion_trigger(&mut code_editor, &event.logical_key, &keys) {
            continue;
        }
        
        // その他のキーの処理
        match &event.logical_key {
            bevy::input::keyboard::Key::Backspace => {
                handle_backspace(&mut code_editor);
            }
            bevy::input::keyboard::Key::Delete => {
                handle_delete(&mut code_editor);
            }
            bevy::input::keyboard::Key::ArrowLeft => {
                handle_arrow_left(&mut code_editor);
            }
            bevy::input::keyboard::Key::ArrowRight => {
                handle_arrow_right(&mut code_editor);
            }
            bevy::input::keyboard::Key::ArrowUp => {
                handle_arrow_up(&mut code_editor);
            }
            bevy::input::keyboard::Key::ArrowDown => {
                handle_arrow_down(&mut code_editor);
            }
            bevy::input::keyboard::Key::Home => {
                handle_home(&mut code_editor);
            }
            bevy::input::keyboard::Key::End => {
                handle_end(&mut code_editor);
            }
            bevy::input::keyboard::Key::Enter => {
                handle_enter(&mut code_editor);
            }
            bevy::input::keyboard::Key::Tab => {
                handle_tab(&mut code_editor);
            }
            _ => {}
        }
    }

    // ショートカットキーの処理
    handle_shortcuts(&mut *code_editor, &mut *error_dialog, &keys);
}
