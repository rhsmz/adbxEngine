use bevy::prelude::*;
use bevy::ui::Interaction;

/// ツールバーのクリック処理
pub fn handle_toolbar_click(
    mut code_editor: ResMut<crate::ui::code_editor::CodeEditor>,
    _script_editor: ResMut<crate::ui::script_editor::ScriptEditor>,
    mut docking: ResMut<crate::ui::docking::DockingSystem>,
    mut file_dialog_request: ResMut<crate::ui::file_dialog::FileDialogRequest>,
    mut error_dialog: ResMut<crate::ui::error_dialog::ErrorDialog>,
    mut communication: ResMut<crate::communication::EditorRuntimeCommunication>,
    mut runtime_state: ResMut<crate::systems::runtime_state::RuntimeStateManager>,
    project: Res<crate::project::Project>,
    scene_manager: Res<crate::systems::scene_management::SceneManager>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                let name_str = name.as_str();
                
                // 新規ファイル
                if name_str == "ToolbarNewFile" {
                    let new_file_index = code_editor.open_files.len() + 1;
                    code_editor.open_files.push(crate::ui::code_editor::OpenFile {
                        path: format!("Untitled-{}", new_file_index),
                        content: String::new(),
                        modified: false,
                    });
                    code_editor.active_tab = code_editor.open_files.len() - 1;
                    code_editor.cursor_position = 0;
                    code_editor.selection_start = None;
                    code_editor.content_entity = None;
                    bevy::log::info!("New file created");
                }
                
                // ファイルを開く
                if name_str == "ToolbarOpenFile" {
                    crate::ui::file_dialog::open_file_dialog(
                        &mut file_dialog_request,
                        "ファイルを開く".to_string(),
                        crate::ui::file_dialog::code_files(),
                    );
                }
                
                // 保存
                if name_str == "ToolbarSave" {
                    crate::ui::code_editor::save_active_file_in_code_editor(&mut *code_editor, &mut *error_dialog);
                }
                
                // Undo
                if name_str == "ToolbarUndo" {
                    crate::ui::code_editor::undo_code_editor_edit(&mut code_editor);
                }
                
                // Redo
                if name_str == "ToolbarRedo" {
                    crate::ui::code_editor::redo_code_editor_edit(&mut code_editor);
                }
                
                // 実行/再生
                if name_str == "ToolbarPlay" {
                    if runtime_state.is_stopped() {
                        // ランタイムプロセスを起動
                        let project_path: Option<&std::path::Path> = project.project_path.as_deref();
                        if let Err(e) = crate::communication::start_runtime_process(
                            &mut communication,
                            project_path,
                        ) {
                            bevy::log::error!("Failed to start runtime process: {}", e);
                            crate::ui::error_dialog::show_error_dialog(
                                &mut *error_dialog,
                                "実行エラー".to_string(),
                                format!("ランタイムプロセスの起動に失敗しました: {}", e),
                                None,
                            );
                        } else {
                            // 現在のシーンを実行
                            let scene_path = project.project_path.as_ref().map(|project_path| {
                                project_path
                                    .join("scenes")
                                    .join(format!("{}.json", scene_manager.current_scene_name))
                                    .to_string_lossy()
                                    .to_string()
                            });
                            runtime_state.start(scene_path);
                            bevy::log::info!("Runtime started");
                        }
                    } else if runtime_state.is_paused() {
                        // 一時停止から再開
                        runtime_state.resume();
                        bevy::log::info!("Runtime resumed");
                    }
                }
                
                // 停止
                if name_str == "ToolbarStop" {
                    if runtime_state.is_playing() || runtime_state.is_paused() {
                        // ランタイムプロセスを停止
                        if let Err(e) = crate::communication::stop_runtime_process(&mut communication) {
                            bevy::log::error!("Failed to stop runtime process: {}", e);
                            crate::ui::error_dialog::show_error_dialog(
                                &mut error_dialog,
                                "停止エラー".to_string(),
                                format!("ランタイムプロセスの停止に失敗しました: {}", e),
                                None,
                            );
                        } else {
                            runtime_state.stop();
                            bevy::log::info!("Runtime stopped");
                        }
                    }
                }
                
                // パネルの表示/非表示切り替え
                if name_str == "ToolbarTogglePanels" {
                    // すべてのパネルの表示/非表示を切り替え
                    for panel_state in docking.panels.values_mut() {
                        panel_state.is_visible = !panel_state.is_visible;
                    }
                    bevy::log::info!("Toggled panel visibility");
                }
            }
        }
    }
    
    // ファイルダイアログの結果を処理
    if let Some(result) = file_dialog_request.result.take() {
        match result {
            crate::ui::file_dialog::FileDialogResult::File(path) => {
                // ファイルを開く
                let path_str = path.to_string_lossy().to_string();
                crate::ui::code_editor::open_file_in_code_editor(path_str, &mut *code_editor, &mut *error_dialog);
            }
            crate::ui::file_dialog::FileDialogResult::Folder(_) => {
                // フォルダ選択の結果（必要に応じて処理）
            }
            crate::ui::file_dialog::FileDialogResult::Cancelled => {
                // キャンセルされた
            }
        }
    }
}
