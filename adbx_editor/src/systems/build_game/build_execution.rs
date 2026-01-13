use super::config_generation::generate_build_config;
use super::resource::{BuildGameRequest, BuildProgress};
use super::workspace_detection::find_workspace_root;
use crate::error::EditorError;
use crate::project::Project;
use bevy::prelude::*;
use std::process::Command;

/// ゲームをビルドする
pub fn build_game(
    project: Res<Project>,
    mut build_request: ResMut<BuildGameRequest>,
    mut build_progress: ResMut<BuildProgress>,
) -> Result<(), EditorError> {
    if build_request.is_building {
        return Err(EditorError::project(
            "Build is already in progress".to_string(),
        ));
    }

    let project_path = project
        .project_path
        .as_ref()
        .ok_or_else(|| EditorError::project("No project path set".to_string()))?;

    build_request.is_building = true;
    build_request.build_output.clear();
    build_request.build_error = None;
    build_progress.is_visible = true;
    build_progress.progress_text = "Preparing build...".to_string();
    build_progress.progress_percent = 0.0;

    // ビルド設定ファイルを生成
    if let Err(e) = generate_build_config(project_path, project.as_ref()) {
        build_request.is_building = false;
        build_progress.is_visible = false;
        return Err(EditorError::project(format!(
            "Failed to generate build config: {}",
            e
        )));
    }

    build_progress.progress_text = "Building game...".to_string();
    build_progress.progress_percent = 10.0;

    // Cargoビルドを実行
    let workspace_root = find_workspace_root(project_path)
        .map_err(|e| EditorError::project(format!("Failed to find workspace root: {}", e)))?;

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("adbx_runtime")
        .current_dir(&workspace_root);

    build_progress.progress_text = "Running cargo build...".to_string();
    build_progress.progress_percent = 20.0;

    let output = cmd
        .output()
        .map_err(|e| EditorError::project(format!("Failed to execute cargo build: {}", e)))?;

    build_progress.progress_percent = 80.0;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        build_request.build_error = Some(error_msg.clone());
        build_request.is_building = false;
        build_progress.is_visible = false;
        return Err(EditorError::project(format!("Build failed: {}", error_msg)));
    }

    build_progress.progress_text = "Packaging game...".to_string();
    build_progress.progress_percent = 90.0;

    // パッケージ化
    let build_output_path = workspace_root.join("target").join("release");
    if let Err(e) = crate::systems::package_game::package_game(project_path, &build_output_path) {
        build_request.is_building = false;
        build_progress.is_visible = false;
        return Err(EditorError::project(format!(
            "Failed to package game: {}",
            e
        )));
    }

    build_progress.progress_text = "Build completed!".to_string();
    build_progress.progress_percent = 100.0;
    build_request.is_building = false;

    bevy::log::info!("Game built successfully!");

    Ok(())
}
