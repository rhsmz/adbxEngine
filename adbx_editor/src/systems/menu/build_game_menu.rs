use bevy::prelude::*;
use crate::project::Project;
use crate::systems::build_game::{BuildGameRequest, BuildProgress, build_game};

/// ビルドゲームリクエスト（リソースベース）
#[derive(Resource, Default)]
pub struct BuildGameMenuRequest {
    pub requested: bool,
}

/// ビルドゲームリクエストを処理
pub fn handle_build_game_request(
    project: Res<Project>,
    mut build_game_request: ResMut<BuildGameMenuRequest>,
    build_request: ResMut<BuildGameRequest>,
    build_progress: ResMut<BuildProgress>,
    mut error_dialog: ResMut<crate::ui::error_dialog::ErrorDialog>,
) {
    if build_game_request.requested {
        build_game_request.requested = false;
        
        // ビルドを実行
        match build_game(
            project,
            build_request,
            build_progress,
        ) {
            Ok(_) => {
                bevy::log::info!("Game built successfully!");
            }
            Err(e) => {
                bevy::log::error!("Build failed: {}", e);
                crate::ui::error_dialog::show_error_dialog_from_error(&mut error_dialog, e);
            }
        }
    }
}
