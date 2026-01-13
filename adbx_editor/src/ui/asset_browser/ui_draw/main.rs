use bevy::prelude::*;
use super::super::{AssetBrowser, AssetType};
use super::header::draw_asset_browser_header;
use super::file_list::draw_asset_file_list;
use super::preview_panel::draw_asset_preview_panel;

/// アセットブラウザーのUI描画
pub fn draw_asset_browser(
    mut commands: Commands,
    mut asset_browser: ResMut<AssetBrowser>,
    asset_server: Res<AssetServer>,
    asset_browser_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    name_query: Query<&Name>,
    images: Res<Assets<Image>>,
) {
    // アセットブラウザーパネルを検索
    let panel_opt = asset_browser_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "AssetBrowserPanel"
        } else {
            false
        }
    });
    
    if let Some(panel) = panel_opt {
        // 変更検知：ディレクトリが変更された場合のみスキャン
        let needs_scan = asset_browser.content_entity.is_none();
        
        if needs_scan {
            asset_browser.scan_directory();
        }
        
        // 総アイテム数を更新
        asset_browser.total_items = asset_browser.asset_files.len();
        
        // 仮想スクロール：表示範囲を計算
        let panel_height = 400.0; // 簡易実装：固定値（実際にはパネルの高さを取得）
        let visible_item_count = (panel_height / asset_browser.item_height).ceil() as usize;
        let scroll_index = (asset_browser.scroll_offset / asset_browser.item_height).floor() as usize;
        let start_index = scroll_index.min(asset_browser.total_items.saturating_sub(1));
        let end_index = (start_index + visible_item_count + 2).min(asset_browser.total_items); // +2はバッファ
        
        asset_browser.visible_range = (start_index, end_index);
        
        // 既存のコンテンツを削除（再構築が必要な場合のみ）
        if needs_scan {
            if let Some(content_entity) = asset_browser.content_entity {
                if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                    entity_commands.despawn();
                }
            }
        }
        
        // コンテンツエンティティが存在しない場合のみ作成
        if asset_browser.content_entity.is_none() {
            let content_entity = commands.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                Name::new("AssetBrowserContent"),
            )).id();
            asset_browser.content_entity = Some(content_entity);
            commands.entity(panel).add_child(content_entity);
        }
        
        // コンテンツエンティティを取得
        if let Some(content_entity) = asset_browser.content_entity {
            commands.entity(content_entity).with_children(|content: &mut ChildSpawnerCommands| {
                // ヘッダー部分
                draw_asset_browser_header(content, &asset_browser);
                
                // アセットリストとプレビューのコンテナ
                content.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    Name::new("AssetBrowserMainContainer"),
                )).with_children(|main_container: &mut ChildSpawnerCommands| {
                    // アセットリスト
                    draw_asset_file_list(main_container, &asset_browser);
                    
                    // プレビューパネル
                    draw_asset_preview_panel(main_container, &mut asset_browser, &asset_server, &images);
                });
            });
        }
    }
}
