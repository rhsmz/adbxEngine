use super::show_functions::hide_rename_dialog;
use super::validation::validate_filename;
use super::RenameDialogRequest;
use bevy::prelude::*;
use bevy::ui::Interaction;

/// リネームダイアログのクリック処理
pub fn handle_rename_dialog_click(
    _commands: Commands,
    mut rename_dialog: ResMut<RenameDialogRequest>,
    mut asset_browser: ResMut<crate::ui::asset_browser::AssetBrowser>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    if !rename_dialog.is_visible {
        return;
    }

    for (interaction, name) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match name.as_str() {
                "RenameDialogCancel" => {
                    hide_rename_dialog(&mut rename_dialog);
                }
                "RenameDialogOK" => {
                    // バリデーション
                    if let Err(e) = validate_filename(&rename_dialog.new_name) {
                        bevy::log::error!("Invalid filename: {}", e);
                        return;
                    }

                    // リネーム処理
                    if let Some(target_path) = &rename_dialog.target_path {
                        let target_path_clone = target_path.clone();
                        let new_path = target_path_clone
                            .parent()
                            .map(|p| p.join(&rename_dialog.new_name))
                            .unwrap_or_else(|| std::path::PathBuf::from(&rename_dialog.new_name));

                        if let Err(e) = std::fs::rename(&target_path_clone, &new_path) {
                            bevy::log::error!("Failed to rename asset: {}", e);
                        } else {
                            bevy::log::info!(
                                "Renamed asset: {:?} -> {:?}",
                                target_path_clone,
                                new_path
                            );
                            rename_dialog.result = Some(super::RenameDialogResult::Renamed {
                                old_path: target_path_clone.clone(),
                                new_path: new_path.clone(),
                            });

                            // アセットブラウザを更新
                            if asset_browser.selected_asset.as_ref() == Some(&target_path_clone) {
                                asset_browser.selected_asset = Some(new_path.clone());
                            }
                            asset_browser.scan_directory();

                            hide_rename_dialog(&mut rename_dialog);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
