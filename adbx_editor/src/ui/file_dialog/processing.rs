use super::resource::{FileDialogRequest, FileDialogResult, FileDialogType};
use bevy::prelude::*;

/// ファイルダイアログの処理システム
#[allow(dead_code)]
pub fn process_file_dialog(mut file_dialog_request: ResMut<FileDialogRequest>) {
    if let Some(dialog_type) = file_dialog_request.dialog_type.take() {
        let result = match dialog_type {
            FileDialogType::OpenFile { title, filters } => {
                let mut dialog = rfd::FileDialog::new().set_title(&title);

                // フィルターを追加
                for filter in filters {
                    let extensions: Vec<&str> =
                        filter.extensions.iter().map(|s| s.as_str()).collect();
                    dialog = dialog.add_filter(&filter.name, &extensions);
                }

                // ファイルを選択
                if let Some(path) = dialog.pick_file() {
                    FileDialogResult::File(path)
                } else {
                    FileDialogResult::Cancelled
                }
            }
            FileDialogType::SaveFile {
                title,
                default_name,
                filters,
            } => {
                let mut dialog = rfd::FileDialog::new().set_title(&title);

                // デフォルトファイル名を設定
                if let Some(name) = default_name {
                    dialog = dialog.set_file_name(&name);
                }

                // フィルターを追加
                for filter in filters {
                    let extensions: Vec<&str> =
                        filter.extensions.iter().map(|s| s.as_str()).collect();
                    dialog = dialog.add_filter(&filter.name, &extensions);
                }

                // ファイルを保存
                if let Some(path) = dialog.save_file() {
                    FileDialogResult::File(path)
                } else {
                    FileDialogResult::Cancelled
                }
            }
            FileDialogType::PickFolder { title } => {
                let dialog = rfd::FileDialog::new().set_title(&title);

                // フォルダを選択
                if let Some(path) = dialog.pick_folder() {
                    FileDialogResult::Folder(path)
                } else {
                    FileDialogResult::Cancelled
                }
            }
        };

        file_dialog_request.result = Some(result);
    }
}
