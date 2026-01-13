use bevy::prelude::*;
use super::super::{AssetBrowser, import_export::{import_asset, export_asset}};

/// ファイルダイアログの結果を処理
pub fn process_file_dialog_result(
    asset_browser: &mut AssetBrowser,
    error_dialog: &mut crate::ui::error_dialog::ErrorDialog,
    result: crate::ui::file_dialog::FileDialogResult,
) {
    match result {
        crate::ui::file_dialog::FileDialogResult::File(path) => {
            if asset_browser.is_export_pending {
                // エクスポート処理
                asset_browser.is_export_pending = false; // フラグをリセット
                if let Some(source_path) = &asset_browser.selected_asset {
                    match export_asset(source_path, path.clone()) {
                        Ok(_) => {
                            bevy::log::info!("Asset exported successfully to: {:?}", path);
                            crate::ui::error_dialog::show_info_dialog(
                                error_dialog,
                                "エクスポート成功".to_string(),
                                format!("アセットをエクスポートしました: {}", path.display()),
                                None,
                            );
                        }
                        Err(e) => {
                            bevy::log::error!("Failed to export asset: {}", e);
                            crate::ui::error_dialog::show_error_dialog(
                                &mut *error_dialog,
                                "エクスポートエラー".to_string(),
                                format!("アセットのエクスポートに失敗しました: {}", e),
                                None,
                            );
                        }
                    }
                }
            } else {
                // インポート処理
                match import_asset(path.clone(), &asset_browser.current_path) {
                    Ok(imported_path) => {
                        bevy::log::info!("Asset imported successfully to: {:?}", imported_path);
                        crate::ui::error_dialog::show_info_dialog(
                            error_dialog,
                            "インポート成功".to_string(),
                            format!("アセットをインポートしました: {}", imported_path.display()),
                            None,
                        );
                        asset_browser.content_entity = None; // UI更新を促す
                        asset_browser.scan_directory(); // ディレクトリを再スキャン
                    }
                    Err(e) => {
                        bevy::log::error!("Failed to import asset: {}", e);
                        crate::ui::error_dialog::show_error_dialog(
                            &mut *error_dialog,
                            "インポートエラー".to_string(),
                            format!("アセットのインポートに失敗しました: {}", e),
                            None,
                        );
                    }
                }
            }
        }
        crate::ui::file_dialog::FileDialogResult::Folder(_) => {
            // フォルダ選択の結果（必要に応じて処理）
            if asset_browser.is_export_pending {
                asset_browser.is_export_pending = false; // フラグをリセット
            }
        }
        crate::ui::file_dialog::FileDialogResult::Cancelled => {
            // キャンセルされた（フラグをリセット）
            if asset_browser.is_export_pending {
                asset_browser.is_export_pending = false;
            }
        }
    }
}
