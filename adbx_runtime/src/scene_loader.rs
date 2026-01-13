use crate::game_config::GameConfig;
use adbx_shared::scene::SceneData;
use bevy::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;

/// シーンに属するエンティティをマークするコンポーネント
#[derive(Component, Debug)]
pub struct SceneEntity {
    pub scene_name: String,
}

/// シーンファイルを読み込む
pub fn load_scene_file(scene_path: &PathBuf) -> Result<SceneData, String> {
    if !scene_path.exists() {
        return Err(format!("Scene file not found: {}", scene_path.display()));
    }

    // まずJSONを試す
    if let Some(ext) = scene_path.extension().and_then(|s| s.to_str()) {
        if ext == "json" {
            let scene_content = std::fs::read_to_string(scene_path)
                .map_err(|e| format!("Failed to read scene file: {}", e))?;

            let scene: SceneData = serde_json::from_str(&scene_content)
                .map_err(|e| format!("Failed to parse scene: {}", e))?;

            return Ok(scene);
        }

        // MessagePackを試す（.msgpackと.mpの両方をサポート）
        if ext == "msgpack" || ext == "mp" {
            // MessagePackサポートは後で実装（rmp-serdeが必要）
            return Err(format!(
                "MessagePack format is not yet supported in runtime"
            ));
        }
    }

    Err(format!(
        "Unsupported scene file format: {}",
        scene_path.display()
    ))
}

/// SceneDataからBevy ECSにシーンを読み込む
pub fn deserialize_scene(
    scene: &SceneData,
    commands: &mut Commands,
    scene_name: &str,
) -> HashMap<u32, Entity> {
    let mut entity_map: HashMap<u32, Entity> = HashMap::new();

    // バッチ処理：まず全てのEntityを作成（親子関係を考慮せず）
    for entity_data in &scene.entities {
        let mut transform = Transform::default();

        // Transformコンポーネントのデータを取得
        for component in &entity_data.components {
            if component.type_name == "Transform" {
                if let Some(translation) = component.data.get("translation") {
                    if let (Some(x), Some(y), Some(z)) = (
                        translation.get("x").and_then(|v| v.as_f64()),
                        translation.get("y").and_then(|v| v.as_f64()),
                        translation.get("z").and_then(|v| v.as_f64()),
                    ) {
                        transform.translation = Vec3::new(x as f32, y as f32, z as f32);
                    }
                }

                if let Some(rotation) = component.data.get("rotation") {
                    if let (Some(x), Some(y), Some(z), Some(w)) = (
                        rotation.get("x").and_then(|v| v.as_f64()),
                        rotation.get("y").and_then(|v| v.as_f64()),
                        rotation.get("z").and_then(|v| v.as_f64()),
                        rotation.get("w").and_then(|v| v.as_f64()),
                    ) {
                        transform.rotation =
                            Quat::from_xyzw(x as f32, y as f32, z as f32, w as f32);
                    }
                }

                if let Some(scale) = component.data.get("scale") {
                    if let (Some(x), Some(y), Some(z)) = (
                        scale.get("x").and_then(|v| v.as_f64()),
                        scale.get("y").and_then(|v| v.as_f64()),
                        scale.get("z").and_then(|v| v.as_f64()),
                    ) {
                        transform.scale = Vec3::new(x as f32, y as f32, z as f32);
                    }
                }
            }
        }

        let entity = commands
            .spawn((
                Name::new(entity_data.name.clone()),
                transform,
                SceneEntity {
                    scene_name: scene_name.to_string(),
                },
            ))
            .id();
        entity_map.insert(entity_data.id, entity);
    }

    // 親子関係を設定（親から子を追加する方法を使用）
    for entity_data in &scene.entities {
        if let (Some(child_entity), Some(parent_id)) =
            (entity_map.get(&entity_data.id), entity_data.parent)
        {
            if let Some(parent_entity) = entity_map.get(&parent_id) {
                commands.entity(*parent_entity).add_child(*child_entity);
            }
        }
    }

    entity_map
}

/// 初期シーンを読み込むシステム
pub fn load_initial_scene(mut commands: Commands, game_config: Res<GameConfig>) {
    if let Some(scene_path) = &game_config.scene_path {
        match load_scene_file(scene_path) {
            Ok(scene_data) => {
                bevy::log::info!("Loading scene: {}", scene_data.name);
                deserialize_scene(&scene_data, &mut commands, &scene_data.name);
                bevy::log::info!(
                    "Scene loaded successfully: {} entities",
                    scene_data.entities.len()
                );
            }
            Err(e) => {
                bevy::log::error!("Failed to load scene: {}", e);
            }
        }
    } else if let Some(project_path) = &game_config.project_path {
        // プロジェクトパスが指定されている場合、デフォルトシーンを探す
        let default_scene_path = project_path.join("scenes").join("MainScene.json");
        if default_scene_path.exists() {
            match load_scene_file(&default_scene_path) {
                Ok(scene_data) => {
                    bevy::log::info!("Loading default scene: {}", scene_data.name);
                    deserialize_scene(&scene_data, &mut commands, &scene_data.name);
                    bevy::log::info!(
                        "Scene loaded successfully: {} entities",
                        scene_data.entities.len()
                    );
                }
                Err(e) => {
                    bevy::log::error!("Failed to load default scene: {}", e);
                }
            }
        } else {
            bevy::log::warn!("No scene specified and default scene not found");
        }
    } else {
        bevy::log::warn!("No scene path or project path specified");
    }
}
