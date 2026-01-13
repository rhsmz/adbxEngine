use crate::project::{load_scene, save_scene_with_format, Project, SerializationFormat};
use crate::systems::operation_recording::record_scene_saved;
use crate::systems::scene_serialization::{deserialize_scene, serialize_scene};
use bevy::prelude::*;
use std::collections::HashMap;

/// シーンに属するエンティティをマークするコンポーネント
#[derive(Component, Debug)]
pub struct SceneEntity {
    pub scene_name: String,
}

/// シーンを読み込むリクエスト（リソースベース）
#[derive(Resource, Default)]
pub struct LoadSceneRequest {
    pub scene_name: Option<String>,
}

/// シーン管理システムのリソース
#[derive(Resource)]
pub struct SceneManager {
    pub entity_id_map: HashMap<u32, Entity>,
    pub current_scene_name: String,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
            entity_id_map: HashMap::new(),
            current_scene_name: "MainScene".to_string(),
        }
    }
}

/// シーンを保存するシステム
pub fn save_current_scene(
    project: &Project,
    scene_manager: &mut ResMut<SceneManager>,
    operation_recorder: Option<&mut ResMut<crate::systems::operation_recording::OperationRecorder>>,
    commands: &Commands,
    entities: &Query<(Entity, &Name), With<Transform>>,
    children_query: &Query<&Children>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
) {
    if let Some(project_path) = &project.project_path {
        let scene_name = scene_manager.current_scene_name.clone();
        let scene_data = serialize_scene(
            scene_name.clone(),
            commands,
            entities,
            children_query,
            transform_query,
            name_query,
        );

        // プロジェクトの設定されたシリアライゼーション形式を使用
        let format = project.serialization_format;
        let scene_path: std::path::PathBuf = project_path.join("scenes").join(format!(
            "{}.{}",
            scene_name,
            match format {
                SerializationFormat::Json => "json",
                SerializationFormat::MessagePack => "msgpack",
            }
        ));

        if let Err(e) = save_scene_with_format(&scene_data, project_path, format) {
            bevy::log::error!("Failed to save scene: {}", e);
        } else {
            bevy::log::info!("Scene saved: {} (format: {:?})", scene_name, format);
            // 操作を記録
            if let Some(recorder) = operation_recorder {
                record_scene_saved(
                    recorder,
                    scene_name.clone(),
                    scene_path.to_string_lossy().to_string(),
                );
            }
        }
    } else {
        bevy::log::warn!("No project path set. Cannot save scene.");
    }
}

/// シーンを読み込むシステム（リソースベース）
pub fn handle_load_scene_request(
    mut commands: Commands,
    project: Res<Project>,
    mut load_scene_request: ResMut<LoadSceneRequest>,
    mut scene_manager: ResMut<SceneManager>,
) {
    if let Some(scene_name) = load_scene_request.scene_name.take() {
        if let Some(project_path) = &project.project_path {
            match load_scene(project_path, &scene_name) {
                Ok(scene_data) => {
                    // 既存のシーンエンティティをクリア
                    // TODO: clear_scene_entities関数を実装する必要があります
                    // 現在は、新しいシーンを読み込む際に既存のエンティティを削除する処理を実装する必要があります
                    // 簡易実装として、SceneEntityコンポーネントを持つエンティティを削除
                    // 実際の実装では、より適切な方法でエンティティを管理する必要があります

                    // 新しいシーンをデシリアライズ
                    let entity_map = deserialize_scene(&scene_data, &mut commands, &scene_name);
                    scene_manager.entity_id_map = entity_map;
                    scene_manager.current_scene_name = scene_name.clone();

                    bevy::log::info!("Scene loaded: {}", scene_name);
                }
                Err(e) => {
                    bevy::log::error!("Failed to load scene: {}", e);
                }
            }
        } else {
            bevy::log::warn!("No project path set. Cannot load scene.");
        }
    }
}

/// 利用可能なシーンのリストを取得
pub fn list_available_scenes(project: &Project) -> Vec<String> {
    if let Some(project_path) = &project.project_path {
        let scenes_dir = project_path.join("scenes");
        if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
            let mut scenes = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "json" || ext == "msgpack" || ext == "mp" {
                            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                                scenes.push(name.to_string());
                            }
                        }
                    }
                }
            }
            scenes.sort();
            return scenes;
        }
    }
    Vec::new()
}
