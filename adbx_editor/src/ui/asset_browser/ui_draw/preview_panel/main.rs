use bevy::prelude::*;
use std::path::PathBuf;
use super::super::super::{AssetBrowser, AssetType};
use super::image_preview::draw_texture_preview;
use super::text_preview::{draw_script_preview, draw_text_preview};
use super::generic_preview::{draw_mesh_preview, draw_generic_preview};
use super::empty_preview::{draw_empty_preview, draw_preview_error};

/// プレビューパネルの描画（画像プレビューなど）
pub fn draw_asset_preview_panel(
    parent: &mut ChildSpawnerCommands,
    asset_browser: &mut AssetBrowser,
    asset_server: &AssetServer,
    images: &bevy::asset::Assets<Image>,
) {
    parent.spawn((
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        Name::new("AssetBrowserPreview"),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
    )).with_children(|preview| {
        // プレビューヘッダー
        preview.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(25.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            Name::new("AssetBrowserPreviewHeader"),
        )).with_children(|header| {
            header.spawn((
                Text::new("Preview"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // プレビューコンテンツ
        let selected_path_opt = asset_browser.selected_asset.clone();
        if let Some(selected_path) = selected_path_opt {
            draw_preview_content(preview, &mut *asset_browser, &selected_path, asset_server, images);
        } else {
            draw_empty_preview(preview);
        }
    });
}

/// プレビューコンテンツの描画
fn draw_preview_content(
    parent: &mut ChildSpawnerCommands,
    asset_browser: &mut AssetBrowser,
    selected_path: &PathBuf,
    asset_server: &AssetServer,
    images: &bevy::asset::Assets<Image>,
) {
    // asset_fileの情報を先に取得（借用を終了させる）
    let asset_file_info = asset_browser.asset_files.iter()
        .find(|f| f.path == *selected_path)
        .map(|f| (f.path.clone(), f.name.clone(), f.asset_type));
    
    if let Some((asset_file_path, asset_file_name, asset_file_type)) = asset_file_info {
        // 画像ハンドルを先に取得（クロージャの外で処理）
        let image_handle_opt = if asset_file_type == AssetType::Texture {
            let image_path_str = if asset_file_path.starts_with("assets/") {
                asset_file_path.strip_prefix("assets/")
                    .ok()
                    .and_then(|p| p.to_str())
                    .map(|s| s.replace('\\', "/"))
            } else {
                asset_file_path.to_str()
                    .map(|s| s.replace('\\', "/"))
            };
            
            if let Some(image_path) = image_path_str {
                Some(asset_browser.image_handles
                    .entry(asset_file_path.clone())
                    .or_insert_with(|| {
                        asset_server.load::<Image>(&image_path)
                    })
                    .clone())
            } else {
                None
            }
        } else {
            None
        };
        
        // 画像が読み込まれているかチェック
        let image_loaded = image_handle_opt.as_ref()
            .map(|handle| images.contains(handle))
            .unwrap_or(false);
        
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                overflow: Overflow::clip_y(),
                ..default()
            },
            Name::new("AssetBrowserPreviewContent"),
        )).with_children(|preview_content| {
            // ファイル情報
            preview_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                Name::new("AssetBrowserPreviewInfo"),
            )).with_children(|info| {
                info.spawn((
                    Text::new(&format!("{}: {}", 
                        match asset_file_type {
                            AssetType::Mesh => "Mesh",
                            AssetType::Texture => "Texture",
                            AssetType::Material => "Material",
                            AssetType::Script => "Script",
                            AssetType::Scene => "Scene",
                            AssetType::Text => "Text",
                            AssetType::Other => "File",
                        },
                        asset_file_name
                    )),
                    bevy::text::TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::WHITE),
                ));
            });
            
            // アセットタイプに応じたプレビュー
            match asset_file_type {
                AssetType::Texture => {
                    draw_texture_preview(preview_content, image_handle_opt, image_loaded);
                }
                AssetType::Script => {
                    draw_script_preview(preview_content, &asset_file_path);
                }
                AssetType::Text => {
                    draw_text_preview(preview_content, &asset_file_path);
                }
                AssetType::Mesh => {
                    draw_mesh_preview(preview_content, &asset_file_path, &asset_file_name);
                }
                _ => {
                    draw_generic_preview(preview_content, &asset_file_path, &asset_file_name, asset_file_type);
                }
            }
        });
    } else {
        draw_preview_error(parent);
    }
}
