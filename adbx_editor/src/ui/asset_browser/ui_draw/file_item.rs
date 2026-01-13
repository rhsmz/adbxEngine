use bevy::prelude::*;
use super::super::{AssetBrowser, AssetFileInfo, AssetType};

/// 個別のファイルアイテムの描画（アイコン、名前、タイプ表示）
pub fn draw_asset_file_item(parent: &mut ChildSpawnerCommands, asset_file: &AssetFileInfo, asset_browser: &AssetBrowser) {
    let is_selected = asset_browser.selected_asset.as_ref()
        .map(|p| p == &asset_file.path)
        .unwrap_or(false);
    
    let icon = if asset_file.is_directory {
        "📁"
    } else {
        match asset_file.asset_type {
            AssetType::Mesh => "🎲",
            AssetType::Texture => "🖼️",
            AssetType::Material => "🎨",
            AssetType::Script => "📜",
            AssetType::Scene => "🎬",
            AssetType::Text => "📝",
            AssetType::Other => "📄",
        }
    };
    
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(2.0)),
            margin: UiRect::vertical(Val::Px(1.0)),
            ..default()
        },
        Name::new(format!("AssetItem_{}", asset_file.name)),
        if is_selected {
            BackgroundColor(Color::srgb(0.2, 0.4, 0.6))
        } else {
            BackgroundColor(Color::NONE)
        },
    )).with_children(|item: &mut ChildSpawnerCommands| {
        // アイコン
        item.spawn((
            Node {
                width: Val::Px(20.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new(format!("AssetIcon_{}", asset_file.name)),
        )).with_children(|icon_node: &mut ChildSpawnerCommands| {
            icon_node.spawn((
                Text::new(icon),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // ファイル名
        item.spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new(format!("AssetName_{}", asset_file.name)),
        )).with_children(|name_node: &mut ChildSpawnerCommands| {
            name_node.spawn((
                Text::new(&asset_file.name),
                bevy::text::TextFont {
                    font_size: 11.0,
                    ..default()
                },
                bevy::text::TextColor(if is_selected {
                    Color::srgb(1.0, 1.0, 1.0)
                } else {
                    Color::srgb(0.9, 0.9, 0.9)
                }),
            ));
        });
    });
}
