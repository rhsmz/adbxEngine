use bevy::prelude::*;
use super::super::AssetBrowser;
use super::file_item::draw_asset_file_item;

/// ファイルリストの描画（仮想スクロール対応）
pub fn draw_asset_file_list(parent: &mut ChildSpawnerCommands, asset_browser: &AssetBrowser) {
    parent.spawn((
        Node {
            width: Val::Percent(60.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(5.0)),
            overflow: Overflow::clip_y(),
            margin: UiRect::right(Val::Px(5.0)),
            ..default()
        },
        Name::new("AssetBrowserList"),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )).with_children(|list: &mut ChildSpawnerCommands| {
        // 仮想スクロール：表示範囲内のアイテムのみ描画
        let (start_index, end_index) = asset_browser.visible_range;
        let visible_files: Vec<_> = asset_browser.asset_files
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= start_index && *i < end_index)
            .map(|(_, file)| file)
            .collect();
        
        for asset_file in visible_files {
            draw_asset_file_item(list, asset_file, asset_browser);
        }
        
        if asset_browser.asset_files.is_empty() {
            list.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Name::new("AssetBrowserEmpty"),
            )).with_children(|empty: &mut ChildSpawnerCommands| {
                empty.spawn((
                    Text::new("No assets found"),
                    bevy::text::TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });
        }
    });
}
