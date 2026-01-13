use bevy::prelude::*;
use super::super::resource::CodeEditor;
use super::super::state::OpenFile;
use super::super::cursor_position::{calculate_cursor_position_from_mouse_click, get_line_number_width, get_line_height, get_padding_left, get_padding_top};

/// コードエディタのクリック処理
pub fn handle_code_editor_click(
    mut code_editor: ResMut<CodeEditor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == bevy::ui::Interaction::Pressed {
                let name_str = name.as_str();
                
                // エディタエリアのクリック（フォーカス）
                if name_str == "CodeEditorTextArea" {
                    handle_text_area_click(&mut code_editor, &windows);
                }
                
                // タブのクリック
                if name_str.starts_with("CodeEditorTab_") {
                    handle_tab_click(&mut code_editor, name_str);
                }
                
                // 新規ファイルボタン
                if name_str == "CodeEditorNewFileButton" {
                    handle_new_file_button_click(&mut code_editor);
                }
            }
        }
    }
}

/// テキストエリアのクリック処理
fn handle_text_area_click(
    code_editor: &mut CodeEditor,
    windows: &Query<&Window>,
) {
    code_editor.is_focused = true;
    
    if let Some(window) = windows.iter().next() {
        if let Some(cursor_pos) = window.cursor_position() {
            let content_text = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
                &active_file.content
            } else {
                &code_editor.content
            };
            
            let editor_x = 0.0;
            let editor_y = 0.0;
            
            let calculated_pos = calculate_cursor_position_from_mouse_click(
                content_text,
                cursor_pos.x - editor_x,
                cursor_pos.y - editor_y,
                get_line_number_width(),
                get_line_height(),
                get_padding_left(),
                get_padding_top(),
            );
            
            code_editor.cursor_position = calculated_pos.min(content_text.len());
        }
    }
    code_editor.content_entity = None;
}

/// タブのクリック処理
fn handle_tab_click(
    code_editor: &mut CodeEditor,
    name_str: &str,
) {
    if let Some(tab_index_str) = name_str.strip_prefix("CodeEditorTab_") {
        if let Ok(tab_index) = tab_index_str.parse::<usize>() {
            if tab_index < code_editor.open_files.len() {
                code_editor.active_tab = tab_index;
                code_editor.cursor_position = 0;
                code_editor.selection_start = None;
                code_editor.content_entity = None;
            }
        }
    }
}

/// 新規ファイルボタンのクリック処理
fn handle_new_file_button_click(code_editor: &mut CodeEditor) {
    let new_file_index = code_editor.open_files.len() + 1;
    code_editor.open_files.push(OpenFile {
        path: format!("Untitled-{}", new_file_index),
        content: String::new(),
        modified: false,
    });
    code_editor.active_tab = code_editor.open_files.len() - 1;
    code_editor.cursor_position = 0;
    code_editor.selection_start = None;
    code_editor.content_entity = None;
}
