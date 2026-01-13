use bevy::prelude::*;
use crate::ui::code_editor::resource::CodeEditor;
use crate::ui::code_editor::state::OpenFile;

/// ファイルを開く
pub fn open_file_in_code_editor(
    file_path: String,
    code_editor: &mut CodeEditor,
    error_dialog: &mut crate::ui::error_dialog::ErrorDialog,
) {
    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            // 既に開いているファイルかチェック
            if let Some(index) = code_editor.open_files.iter().position(|f| f.path == file_path) {
                code_editor.active_tab = index;
            } else {
                code_editor.open_files.push(OpenFile {
                    path: file_path.clone(),
                    content,
                    modified: false,
                });
                code_editor.active_tab = code_editor.open_files.len() - 1;
            }
            code_editor.current_file = Some(file_path);
            code_editor.content_entity = None;
        }
        Err(e) => {
            bevy::log::error!("Failed to open file: {}", e);
            crate::ui::error_dialog::show_error_dialog(
                error_dialog,
                "ファイルを開けませんでした".to_string(),
                format!("ファイル '{}' を開くことができませんでした。", file_path),
                Some(format!("エラー詳細: {}", e)),
            );
        }
    }
}

/// 現在のファイルを保存
pub fn save_active_file_in_code_editor(
    code_editor: &mut CodeEditor,
    error_dialog: &mut crate::ui::error_dialog::ErrorDialog,
) {
    let active_tab = code_editor.active_tab;
    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        if let Err(e) = std::fs::write(&active_file.path, &active_file.content) {
            bevy::log::error!("Failed to save file: {}", e);
            crate::ui::error_dialog::show_error_dialog(
                error_dialog,
                "ファイルを保存できませんでした".to_string(),
                format!("ファイル '{}' を保存することができませんでした。", active_file.path),
                Some(format!("エラー詳細: {}", e)),
            );
        } else {
            active_file.modified = false;
            bevy::log::info!("File saved: {}", active_file.path);
        }
    } else {
        crate::ui::error_dialog::show_warning_dialog(
            error_dialog,
            "保存するファイルがありません".to_string(),
            "現在開いているファイルがありません。".to_string(),
            None,
        );
    }
}
