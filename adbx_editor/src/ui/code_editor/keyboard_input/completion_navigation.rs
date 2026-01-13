use super::super::resource::CodeEditor;
use super::super::completion::{trigger_code_completion_for_current_position, accept_selected_completion_candidate};

/// 補完ナビゲーションの処理
pub fn handle_completion_navigation(
    code_editor: &mut CodeEditor,
    key: &bevy::input::keyboard::Key,
) -> bool {
    if !code_editor.completion_state.is_visible {
        return false;
    }
    
    match key {
        bevy::input::keyboard::Key::ArrowUp => {
            if code_editor.completion_state.selected_index > 0 {
                code_editor.completion_state.selected_index -= 1;
                code_editor.content_entity = None;
            }
            true
        }
        bevy::input::keyboard::Key::ArrowDown => {
            if code_editor.completion_state.selected_index < code_editor.completion_state.candidates.len().saturating_sub(1) {
                code_editor.completion_state.selected_index += 1;
                code_editor.content_entity = None;
            }
            true
        }
        bevy::input::keyboard::Key::Enter | bevy::input::keyboard::Key::Tab => {
            accept_selected_completion_candidate(code_editor);
            true
        }
        bevy::input::keyboard::Key::Escape => {
            code_editor.completion_state.is_visible = false;
            code_editor.content_entity = None;
            true
        }
        _ => false,
    }
}

/// Ctrl+Spaceで補完を手動でトリガー
pub fn handle_manual_completion_trigger(
    code_editor: &mut CodeEditor,
    key: &bevy::input::keyboard::Key,
    keys: &bevy::prelude::Res<bevy::prelude::ButtonInput<bevy::prelude::KeyCode>>,
) -> bool {
    if keys.pressed(bevy::prelude::KeyCode::ControlLeft) || keys.pressed(bevy::prelude::KeyCode::ControlRight) {
        if let bevy::input::keyboard::Key::Character(ch) = key {
            if ch == " " {
                trigger_code_completion_for_current_position(code_editor);
                code_editor.content_entity = None;
                return true;
            }
        }
    }
    false
}
