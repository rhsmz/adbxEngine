use crate::project::{create_project, load_project, Project};
use crate::settings::editor_settings::SettingsPanel;
use bevy::prelude::*;
use bevy::ui::Interaction;
use std::path::PathBuf;

/// プロジェクト管理のリクエスト（リソースベース）
#[derive(Resource, Default)]
pub struct ProjectRequest {
    pub new_project_path: Option<PathBuf>,
    pub open_project_path: Option<PathBuf>,
}

/// メニューのクリック処理
pub fn handle_menu_click(
    mut project: ResMut<Project>,
    _project_request: ResMut<ProjectRequest>,
    mut file_dialog_request: ResMut<crate::ui::file_dialog::FileDialogRequest>,
    mut settings_panel: ResMut<SettingsPanel>,
    mut build_game_request: ResMut<crate::systems::menu::build_game_menu::BuildGameMenuRequest>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                let name_str = name.as_str();

                // 新規プロジェクト
                if name_str == "MenuNewProject" {
                    // 簡易実装：デフォルトパスで新規プロジェクトを作成
                    let default_path = PathBuf::from("projects").join("NewProject");
                    if let Ok(new_project) =
                        create_project("New Project".to_string(), default_path.clone())
                    {
                        *project = new_project;
                        bevy::log::info!("New project created: {:?}", default_path);
                    } else {
                        bevy::log::error!("Failed to create new project");
                    }
                }

                // プロジェクトを開く
                if name_str == "MenuOpenProject" {
                    // プロジェクトフォルダを選択
                    crate::ui::file_dialog::pick_folder_dialog(
                        &mut file_dialog_request,
                        "プロジェクトを開く".to_string(),
                    );
                }

                // 設定パネルを開く
                if name_str == "MenuSettings" {
                    settings_panel.is_open = true;
                }

                // ゲームをビルド
                if name_str == "MenuBuildGame" {
                    build_game_request.requested = true;
                    bevy::log::info!("Build game requested from menu");
                }
            }
        }
    }

    // ファイルダイアログの結果を処理
    if let Some(result) = file_dialog_request.result.take() {
        match result {
            crate::ui::file_dialog::FileDialogResult::Folder(path) => {
                // プロジェクトを読み込む
                if let Ok(loaded_project) = load_project(path.clone()) {
                    *project = loaded_project;
                    bevy::log::info!("Project loaded: {:?}", path);
                } else {
                    bevy::log::error!("Failed to load project: {:?}", path);
                }
            }
            _ => {}
        }
    }
}
