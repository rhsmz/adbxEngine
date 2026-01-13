use super::super::{handle_asset_operations, AssetOperation};
use crate::ui::asset_browser::AssetBrowser;
use crate::ui::file_dialog::FileDialogRequest;
use crate::ui::rename_dialog::RenameDialogRequest;
use bevy::prelude::*;

/// アセット操作のメニュー項目を処理
pub fn handle_asset_operation_menu_item(
    asset_browser: &mut ResMut<AssetBrowser>,
    file_dialog_request: &mut ResMut<FileDialogRequest>,
    rename_dialog: &mut ResMut<RenameDialogRequest>,
    operation: &str,
) {
    let asset_operation = match operation {
        "ContextMenuImportAsset" => AssetOperation::Import,
        "ContextMenuExportAsset" => AssetOperation::Export,
        "ContextMenuDeleteAsset" => AssetOperation::Delete,
        "ContextMenuRenameAsset" => AssetOperation::Rename,
        "ContextMenuShowInExplorer" => AssetOperation::ShowInExplorer,
        _ => return,
    };

    handle_asset_operations(
        asset_browser,
        file_dialog_request,
        rename_dialog,
        asset_operation,
    );
}
