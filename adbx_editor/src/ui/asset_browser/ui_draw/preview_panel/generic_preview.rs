use bevy::prelude::*;
use std::path::PathBuf;
use super::super::super::AssetType;

/// メッシュプレビューの描画
pub fn draw_mesh_preview(parent: &mut ChildSpawnerCommands, asset_file_path: &PathBuf, asset_file_name: &str) {
    if let Ok(metadata) = std::fs::metadata(asset_file_path) {
        let file_size = metadata.len();
        let file_size_str = if file_size < 1024 {
            format!("{} B", file_size)
        } else if file_size < 1024 * 1024 {
            format!("{} KB", file_size / 1024)
        } else {
            format!("{} MB", file_size / (1024 * 1024))
        };
        
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            Name::new("AssetBrowserMeshPreview"),
        )).with_children(|mesh_preview: &mut ChildSpawnerCommands| {
            // 3Dモデルアイコン
            mesh_preview.spawn((
                Text::new("🎲"),
                bevy::text::TextFont {
                    font_size: 48.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
            
            // ファイル情報
            mesh_preview.spawn((
                Node {
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                Name::new("AssetBrowserMeshInfo"),
            )).with_children(|info: &mut ChildSpawnerCommands| {
                info.spawn((
                    Text::new(&format!(
                        "3Dモデル\n\nファイル名: {}\n\nサイズ: {}\n\n形式: {}",
                        asset_file_name,
                        file_size_str,
                        asset_file_path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("Unknown")
                            .to_uppercase()
                    )),
                    bevy::text::TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
        });
    }
}

/// 汎用プレビューの描画
pub fn draw_generic_preview(
    parent: &mut ChildSpawnerCommands,
    asset_file_path: &PathBuf,
    asset_file_name: &str,
    asset_file_type: AssetType,
) {
    if let Ok(metadata) = std::fs::metadata(asset_file_path) {
        let file_size = metadata.len();
        let file_size_str = if file_size < 1024 {
            format!("{} B", file_size)
        } else if file_size < 1024 * 1024 {
            format!("{} KB", file_size / 1024)
        } else {
            format!("{} MB", file_size / (1024 * 1024))
        };
        
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            Name::new("AssetBrowserFileInfo"),
        )).with_children(|file_info: &mut ChildSpawnerCommands| {
            file_info.spawn((
                Text::new(&format!(
                    "ファイル名: {}\n\nサイズ: {}\n\nタイプ: {}",
                    asset_file_name,
                    file_size_str,
                    match asset_file_type {
                        AssetType::Mesh => "3Dモデル",
                        AssetType::Material => "マテリアル",
                        AssetType::Scene => "シーン",
                        _ => "その他",
                    }
                )),
                bevy::text::TextFont {
                    font_size: 11.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
    }
}
