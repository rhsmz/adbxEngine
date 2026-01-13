use crate::ui::asset_browser::AssetBrowser;
use crate::ui::context_menu::AssetOperation;
use crate::ui::file_dialog::FileDialogRequest;
use crate::ui::rename_dialog::RenameDialogRequest;
use bevy::prelude::*;

pub fn handle_asset_operations(
    asset_browser: &mut ResMut<AssetBrowser>,
    file_dialog_request: &mut ResMut<FileDialogRequest>,
    rename_dialog: &mut ResMut<RenameDialogRequest>,
    operation: AssetOperation,
) {
    match operation {
        AssetOperation::Import => {
            crate::ui::file_dialog::open_file_dialog(
                file_dialog_request,
                "アセットをインポート".to_string(),
                crate::ui::file_dialog::all_files(),
            );
        }
        AssetOperation::Export => {
            if let Some(selected_asset) = &asset_browser.selected_asset {
                let default_name = selected_asset
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string());
                crate::ui::file_dialog::save_file_dialog(
                    file_dialog_request,
                    "アセットをエクスポート".to_string(),
                    default_name,
                    crate::ui::file_dialog::all_files(),
                );
            } else {
                bevy::log::warn!("No asset selected. Cannot export asset.");
            }
        }
        AssetOperation::Delete => {
            if let Some(selected_asset) = &asset_browser.selected_asset {
                if selected_asset.exists() {
                    if selected_asset.is_file() {
                        if let Err(e) = std::fs::remove_file(selected_asset) {
                            bevy::log::error!("Failed to delete asset: {}", e);
                        } else {
                            bevy::log::info!("Deleted asset: {:?}", selected_asset);
                            asset_browser.selected_asset = None;
                            asset_browser.scan_directory();
                        }
                    } else if selected_asset.is_dir() {
                        if let Err(e) = std::fs::remove_dir_all(selected_asset) {
                            bevy::log::error!("Failed to delete asset directory: {}", e);
                        } else {
                            bevy::log::info!("Deleted asset directory: {:?}", selected_asset);
                            asset_browser.selected_asset = None;
                            asset_browser.scan_directory();
                        }
                    }
                }
            } else {
                bevy::log::warn!("No asset selected. Cannot delete asset.");
            }
        }
        AssetOperation::Rename => {
            if let Some(selected_asset) = &asset_browser.selected_asset.clone() {
                crate::ui::rename_dialog::show_rename_dialog(rename_dialog, selected_asset.clone());
            } else {
                bevy::log::warn!("No asset selected. Cannot rename asset.");
            }
        }
        AssetOperation::ShowInExplorer => {
            if let Some(selected_asset) = &asset_browser.selected_asset {
                #[cfg(target_os = "windows")]
                {
                    use std::process::Command;
                    if let Some(_parent) = selected_asset.parent() {
                        let _ = Command::new("explorer")
                            .arg("/select,")
                            .arg(selected_asset)
                            .spawn();
                    } else {
                        let _ = Command::new("explorer").arg(selected_asset).spawn();
                    }
                }
                #[cfg(target_os = "macos")]
                {
                    use std::process::Command;
                    let _ = Command::new("open").arg("-R").arg(selected_asset).spawn();
                }
                #[cfg(target_os = "linux")]
                {
                    use std::process::Command;
                    let _ = Command::new("xdg-open")
                        .arg(selected_asset.parent().unwrap_or(selected_asset))
                        .spawn();
                }
                bevy::log::info!("Opened in explorer: {:?}", selected_asset);
            } else {
                bevy::log::warn!("No asset selected. Cannot show in explorer.");
            }
        }
    }
}
