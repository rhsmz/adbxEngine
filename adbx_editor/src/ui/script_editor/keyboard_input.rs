use bevy::prelude::*;
use super::{ScriptEditor, validation::validate_script};
use super::cursor_position::{get_cursor_position, get_char_position, get_line_length, insert_char_at_position, remove_char_at_position};

/// スクリプトエディタのキーボード入力処理
pub fn handle_script_editor_keyboard_input(
    mut script_editor: ResMut<ScriptEditor>,
    mut keyboard_input: MessageReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // エディタがフォーカスされていない場合は処理しない
    if !script_editor.is_focused {
        return;
    }

    // キーボードイベントの処理
    for event in keyboard_input.read() {
        if !event.state.is_pressed() {
            continue;
        }

        // 文字入力の処理
        if let bevy::input::keyboard::Key::Character(ch) = &event.logical_key {
            // 制御文字は無視
            if ch.chars().next().map(|c| c.is_control()).unwrap_or(false) {
                continue;
            }

            // 文字を挿入
            let cursor_pos = script_editor.cursor_position;
            insert_char_at_position(&mut script_editor.content, cursor_pos, ch.chars().next().unwrap());
            script_editor.cursor_position = cursor_pos + ch.len();

            // UI更新を促す
            script_editor.content_entity = None;
            
            // リアルタイムでエラーを検出（簡易実装：短い遅延後に検証）
            // 実際の実装では、デバウンス処理を追加することを推奨
            validate_script(script_editor.as_mut());
            
            continue;
        }

        // その他のキーの処理
        match &event.logical_key {
            bevy::input::keyboard::Key::Backspace => {
                let cursor_pos = script_editor.cursor_position;
                if cursor_pos > 0 {
                    // カーソル位置の前の文字を削除
                    remove_char_at_position(&mut script_editor.content, cursor_pos - 1);
                    script_editor.cursor_position = cursor_pos - 1;
                    script_editor.content_entity = None;
                    // リアルタイムでエラーを検出
                    validate_script(script_editor.as_mut());
                }
            }
            bevy::input::keyboard::Key::Delete => {
                let cursor_pos = script_editor.cursor_position;
                if cursor_pos < script_editor.content.len() {
                    // カーソル位置の文字を削除
                    remove_char_at_position(&mut script_editor.content, cursor_pos);
                    script_editor.content_entity = None;
                    // リアルタイムでエラーを検出
                    validate_script(script_editor.as_mut());
                }
            }
            bevy::input::keyboard::Key::ArrowLeft => {
                if script_editor.cursor_position > 0 {
                    script_editor.cursor_position -= 1;
                    script_editor.content_entity = None;
                }
            }
            bevy::input::keyboard::Key::ArrowRight => {
                if script_editor.cursor_position < script_editor.content.len() {
                    script_editor.cursor_position += 1;
                    script_editor.content_entity = None;
                }
            }
            bevy::input::keyboard::Key::ArrowUp => {
                let (current_line, current_col) = get_cursor_position(&script_editor.content, script_editor.cursor_position);
                if current_line > 0 {
                    let new_line = current_line - 1;
                    let new_col = current_col.min(get_line_length(&script_editor.content, new_line));
                    script_editor.cursor_position = get_char_position(&script_editor.content, new_line, new_col);
                    script_editor.content_entity = None;
                }
            }
            bevy::input::keyboard::Key::ArrowDown => {
                let (current_line, current_col) = get_cursor_position(&script_editor.content, script_editor.cursor_position);
                let total_lines = script_editor.content.lines().count();
                if current_line < total_lines - 1 {
                    let new_line = current_line + 1;
                    let new_col = current_col.min(get_line_length(&script_editor.content, new_line));
                    script_editor.cursor_position = get_char_position(&script_editor.content, new_line, new_col);
                    script_editor.content_entity = None;
                }
            }
            bevy::input::keyboard::Key::Home => {
                let (current_line, _) = get_cursor_position(&script_editor.content, script_editor.cursor_position);
                script_editor.cursor_position = get_char_position(&script_editor.content, current_line, 0);
                script_editor.content_entity = None;
            }
            bevy::input::keyboard::Key::End => {
                let (current_line, _) = get_cursor_position(&script_editor.content, script_editor.cursor_position);
                let line_length = get_line_length(&script_editor.content, current_line);
                script_editor.cursor_position = get_char_position(&script_editor.content, current_line, line_length);
                script_editor.content_entity = None;
            }
            bevy::input::keyboard::Key::Enter => {
                let cursor_pos = script_editor.cursor_position;
                insert_char_at_position(&mut script_editor.content, cursor_pos, '\n');
                script_editor.cursor_position = cursor_pos + 1;
                script_editor.content_entity = None;
            }
            bevy::input::keyboard::Key::Tab => {
                // タブ文字を挿入（スペース4つ）
                let mut cursor_pos = script_editor.cursor_position;
                for _ in 0..4 {
                    insert_char_at_position(&mut script_editor.content, cursor_pos, ' ');
                    cursor_pos += 1;
                }
                script_editor.cursor_position = cursor_pos;
                script_editor.content_entity = None;
            }
            _ => {}
        }
    }

    // Ctrlキーとの組み合わせ
    if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
        if keys.just_pressed(KeyCode::KeyS) {
            // 保存
            if let Some(script_path) = &script_editor.current_script {
                if let Err(e) = std::fs::write(script_path, &script_editor.content) {
                    bevy::log::error!("Failed to save script: {}", e);
                } else {
                    bevy::log::info!("Script saved: {:?}", script_path);
                }
            } else {
                bevy::log::warn!("No script file selected. Cannot save.");
            }
        }
    }
}
