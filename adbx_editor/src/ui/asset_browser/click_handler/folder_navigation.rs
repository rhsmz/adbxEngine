use super::super::AssetBrowser;
use bevy::prelude::*;

/// フォルダ移動処理
pub fn handle_folder_navigation(asset_browser: &mut AssetBrowser, file_name: &str) {
    // アセットファイルを検索（パスを先に取得）
    let asset_path_opt = asset_browser
        .asset_files
        .iter()
        .find(|f| f.name == file_name)
        .map(|f| (f.is_directory, f.path.clone()));

    if let Some((is_directory, asset_path)) = asset_path_opt {
        if is_directory {
            // ディレクトリの場合は移動
            asset_browser.current_path = asset_path;
            asset_browser.selected_asset = None;
        }
    }
}

/// 戻るボタンのクリック処理
pub fn handle_back_button_click(asset_browser: &mut AssetBrowser) {
    if let Some(parent) = asset_browser.current_path.parent() {
        asset_browser.current_path = parent.to_path_buf();
        asset_browser.selected_asset = None;
    }
}
