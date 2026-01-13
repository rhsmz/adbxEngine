use bevy::prelude::*;
use std::path::PathBuf;
use super::RenameDialogRequest;

/// リネームダイアログを表示
pub fn show_rename_dialog(
    rename_dialog: &mut ResMut<RenameDialogRequest>,
    target_path: PathBuf,
) {
    let current_name = target_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    rename_dialog.is_visible = true;
    rename_dialog.target_path = Some(target_path);
    rename_dialog.current_name = current_name.clone();
    rename_dialog.new_name = current_name;
    rename_dialog.result = None;
}

/// リネームダイアログを閉じる
pub fn hide_rename_dialog(rename_dialog: &mut ResMut<RenameDialogRequest>) {
    rename_dialog.is_visible = false;
    rename_dialog.target_path = None;
    rename_dialog.current_name.clear();
    rename_dialog.new_name.clear();
    rename_dialog.result = None;
}
