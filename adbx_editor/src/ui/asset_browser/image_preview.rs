use super::AssetBrowser;
use bevy::prelude::*;

/// 画像の読み込み状態を監視し、読み込まれたらUIを更新
pub fn update_image_previews(mut asset_browser: ResMut<AssetBrowser>, images: Res<Assets<Image>>) {
    // 画像が読み込まれたかチェック
    if asset_browser.pending_image_updates {
        let mut all_loaded = true;
        for handle in asset_browser.image_handles.values() {
            if !images.contains(handle) {
                all_loaded = false;
                break;
            }
        }

        if all_loaded {
            // すべての画像が読み込まれたので、UIを更新
            asset_browser.pending_image_updates = false;
            asset_browser.content_entity = None; // UI再描画を促す
        }
    }
}
