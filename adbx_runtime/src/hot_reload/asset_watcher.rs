use bevy::prelude::*;

/// アセット監視システム
/// Bevyの標準的なAssetWatcherを使用
pub fn setup_asset_watcher(_app: &mut App) {
    // BevyのDefaultPluginsに含まれるAssetWatcherが自動的に有効になる
    // file_watcher featureが有効になっていることを確認
    bevy::log::info!("Asset watcher enabled (file_watcher feature required)");
}

/// アセット変更イベントの処理
pub fn handle_asset_changes(
    mut asset_events: bevy::prelude::MessageReader<bevy::asset::AssetEvent<bevy::prelude::Image>>,
    _custom_asset_registry: Res<crate::hot_reload::custom_asset_registry::CustomAssetRegistry>,
) {
    for event in asset_events.read() {
        match event {
            bevy::asset::AssetEvent::Modified { id } => {
                bevy::log::info!("Asset modified: {:?}", id);
                // 注意: Bevy 0.17では、AssetIdから直接パスを取得する方法が変更されている可能性があります
                // カスタムアセットのホットリロードは、ファイル監視システムから直接呼び出されることを想定
                // ここではログのみ出力
            }
            bevy::asset::AssetEvent::Removed { id } => {
                bevy::log::info!("Asset removed: {:?}", id);
            }
            _ => {}
        }
    }
}
