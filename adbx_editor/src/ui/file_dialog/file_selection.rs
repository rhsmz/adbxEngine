use bevy::prelude::*;
use super::resource::{FileDialogRequest, FileDialogType, FileDialogResult, FileFilter};

/// ファイルダイアログを開く（ファイルを開く）
pub fn open_file_dialog(
    file_dialog_request: &mut FileDialogRequest,
    title: String,
    filters: Vec<FileFilter>,
) {
    file_dialog_request.dialog_type = Some(FileDialogType::OpenFile { title, filters });
}

/// ファイルダイアログを開く（ファイルを保存）
pub fn save_file_dialog(
    file_dialog_request: &mut FileDialogRequest,
    title: String,
    default_name: Option<String>,
    filters: Vec<FileFilter>,
) {
    file_dialog_request.dialog_type = Some(FileDialogType::SaveFile {
        title,
        default_name,
        filters,
    });
}

/// フォルダ選択ダイアログを開く
pub fn pick_folder_dialog(
    file_dialog_request: &mut ResMut<FileDialogRequest>,
    title: String,
) {
    file_dialog_request.dialog_type = Some(FileDialogType::PickFolder { title });
}
