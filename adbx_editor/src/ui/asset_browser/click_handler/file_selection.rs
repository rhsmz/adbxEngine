use bevy::prelude::*;
use super::super::{AssetBrowser, AssetType};

/// ファイル選択処理
pub fn handle_file_selection(
    asset_browser: &mut AssetBrowser,
    script_editor: &mut crate::ui::script_editor::ScriptEditor,
    file_name: &str,
) {
    // アセットファイルを検索（パスを先に取得）
    let asset_path_opt = asset_browser.asset_files.iter()
        .find(|f| f.name == file_name)
        .map(|f| (f.is_directory, f.asset_type, f.path.clone()));
    
    if let Some((is_directory, asset_type, asset_path)) = asset_path_opt {
        if !is_directory {
            // ファイルの場合は選択
            asset_browser.selected_asset = Some(asset_path.clone());
            
            // スクリプトファイルの場合は、スクリプトエディタで開く
            if asset_type == AssetType::Script {
                crate::ui::script_editor::load_script_file(asset_path, &mut script_editor);
            }
        }
    }
}

/// インポートボタンのクリック処理
pub fn handle_import_button_click(
    file_dialog_request: &mut crate::ui::file_dialog::FileDialogRequest,
) {
    crate::ui::file_dialog::open_file_dialog(
        &mut file_dialog_request,
        "アセットをインポート".to_string(),
        crate::ui::file_dialog::all_files(),
    );
}

/// エクスポートボタンのクリック処理
pub fn handle_export_button_click(
    asset_browser: &mut AssetBrowser,
    file_dialog_request: &mut crate::ui::file_dialog::FileDialogRequest,
    error_dialog: &mut crate::ui::error_dialog::ErrorDialog,
) {
    if let Some(selected_path) = asset_browser.selected_asset.clone() {
        // エクスポート待ちフラグを立てる
        asset_browser.is_export_pending = true;
        // エクスポート先を選択
        let default_name = selected_path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());
        crate::ui::file_dialog::save_file_dialog(
            &mut file_dialog_request,
            "アセットをエクスポート".to_string(),
            default_name,
            crate::ui::file_dialog::all_files(),
        );
    } else {
        crate::ui::error_dialog::show_warning_dialog(
            &mut error_dialog,
            "エクスポートエラー".to_string(),
            "エクスポートするアセットが選択されていません。".to_string(),
            None,
        );
    }
}
