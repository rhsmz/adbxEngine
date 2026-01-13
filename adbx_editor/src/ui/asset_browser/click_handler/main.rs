use super::super::AssetBrowser;
use super::click_detection::detect_click_event;
use super::dialog_processing::process_file_dialog_result;
use super::file_selection::{
    handle_export_button_click, handle_file_selection, handle_import_button_click,
};
use super::folder_navigation::{handle_back_button_click, handle_folder_navigation};
use bevy::prelude::*;
use bevy::ui::Interaction;

/// アセットブラウザーのクリック処理
pub fn handle_asset_browser_click(
    mut asset_browser: ResMut<AssetBrowser>,
    mut script_editor: ResMut<crate::ui::script_editor::ScriptEditor>,
    mut file_dialog_request: ResMut<crate::ui::file_dialog::FileDialogRequest>,
    mut error_dialog: ResMut<crate::ui::error_dialog::ErrorDialog>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    // クリックイベントの検出
    let clicked_items = detect_click_event(&mouse_input, &interaction_query);

    for (name_str, _interaction) in clicked_items {
        // 戻るボタン
        if name_str == "AssetBrowserBackButton" {
            handle_back_button_click(&mut asset_browser);
        }

        // インポートボタン
        if name_str == "AssetBrowserImportButton" {
            handle_import_button_click(&mut file_dialog_request);
        }

        // エクスポートボタン
        if name_str == "AssetBrowserExportButton" {
            handle_export_button_click(
                &mut asset_browser,
                &mut file_dialog_request,
                &mut error_dialog,
            );
        }

        // アセットアイテムがクリックされた場合
        if name_str.starts_with("AssetItem_") {
            let file_name = name_str.strip_prefix("AssetItem_").unwrap_or("");

            // まずフォルダ移動を試みる
            handle_folder_navigation(&mut asset_browser, file_name);

            // ファイル選択を試みる
            handle_file_selection(&mut asset_browser, &mut script_editor, file_name);
        }
    }

    // ファイルダイアログの結果を処理
    if let Some(result) = file_dialog_request.result.take() {
        process_file_dialog_result(&mut asset_browser, &mut error_dialog, result);
    }
}
