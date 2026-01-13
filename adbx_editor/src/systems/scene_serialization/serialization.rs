use adbx_shared::scene::{ComponentData, EntityData, SceneData};
use bevy::prelude::*;

/// 現在のシーンをSceneDataにシリアライズ
pub fn serialize_scene(
    name: String,
    _commands: &Commands,
    entities: &Query<(Entity, &Name), With<Transform>>,
    children_query: &Query<&Children>,
    transform_query: &Query<&Transform>,
    _name_query: &Query<&Name>,
) -> SceneData {
    // Entity IDのマッピングを作成
    let entity_map = create_entity_id_mapping(entities);

    // 親子関係の逆引きマップを作成
    let child_to_parent = create_child_to_parent_map(entities, children_query);

    let mut entity_data_list: Vec<EntityData> = Vec::new();

    // 各Entityのデータを収集
    for (entity, entity_name) in entities.iter() {
        let entity_id = entity_map[&entity];
        let parent_id = child_to_parent
            .get(&entity)
            .and_then(|p| entity_map.get(p).copied());

        let mut components = Vec::new();

        // Transformコンポーネントをシリアライズ
        if let Ok(transform) = transform_query.get(entity) {
            components.push(ComponentData {
                type_name: "Transform".to_string(),
                data: serde_json::json!({
                    "translation": {
                        "x": transform.translation.x,
                        "y": transform.translation.y,
                        "z": transform.translation.z,
                    },
                    "rotation": {
                        "x": transform.rotation.x,
                        "y": transform.rotation.y,
                        "z": transform.rotation.z,
                        "w": transform.rotation.w,
                    },
                    "scale": {
                        "x": transform.scale.x,
                        "y": transform.scale.y,
                        "z": transform.scale.z,
                    },
                }),
            });
        }

        // NameコンポーネントはEntityDataに含まれているので、ここでは追加しない

        entity_data_list.push(EntityData {
            id: entity_id,
            name: entity_name.to_string(),
            parent: parent_id,
            components,
        });
    }

    SceneData {
        name,
        entities: entity_data_list,
    }
}
