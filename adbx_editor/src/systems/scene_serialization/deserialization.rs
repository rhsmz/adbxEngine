use adbx_shared::components::SceneEntity;
use adbx_shared::scene::{ComponentData, SceneData};
use bevy::prelude::*;
use std::collections::HashMap;

/// Transformコンポーネントをデシリアライズ
fn deserialize_transform_component(component: &ComponentData) -> Transform {
    let mut transform = Transform::default();

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
            transform.rotation = Quat::from_xyzw(x as f32, y as f32, z as f32, w as f32);
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

    transform
}

/// SceneDataからBevy ECSにシーンを読み込む（バッチ処理対応）
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
                transform = deserialize_transform_component(component);
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
