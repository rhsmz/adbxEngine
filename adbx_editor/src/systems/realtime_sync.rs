use bevy::prelude::*;
use crate::communication::{EditorRuntimeCommunication, notify_entity_selection, notify_component_update};
use crate::systems::selection::Selection;
use adbx_shared::scene::ComponentData;

/// リアルタイム同期システム
/// Entity状態、Component変更、選択状態をランタイムと同期します

/// Entity選択状態をランタイムに同期
pub fn sync_entity_selection(
    communication: Res<EditorRuntimeCommunication>,
    selection: Res<Selection>,
) {
    // 選択状態が変更された場合のみ同期
    if !selection.selected_entities.is_empty() {
        notify_entity_selection(&communication, &selection.selected_entities);
    }
}

/// Transform変更をランタイムに同期
pub fn sync_transform_changes(
    communication: Res<EditorRuntimeCommunication>,
    transform_query: Query<(Entity, &Transform), Changed<Transform>>,
    _name_query: Query<&Name>,
) {
    for (entity, transform) in transform_query.iter() {
        // TransformをComponentDataに変換
        let component_data = ComponentData {
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
        };
        
        notify_component_update(&communication, entity, component_data);
    }
}

/// ランタイムからのEntity状態更新をエディタに反映
pub fn apply_runtime_entity_updates(
    mut communication: ResMut<EditorRuntimeCommunication>,
    mut transform_query: Query<(Entity, &mut Transform)>,
    _commands: Commands,
) {
    let messages = crate::communication::receive_from_runtime(communication.as_mut());
    
    for message in messages {
        match message {
            adbx_shared::RuntimeMessage::EntityUpdated { entity_id } => {
                // Entity IDからEntityを検索して更新
                // 注意: 実際の実装では、Entity IDマッピングを使用する必要があります
                for (entity, _transform) in transform_query.iter_mut() {
                    if entity.index() == entity_id {
                        // ランタイムからの更新を反映（簡易実装）
                        // 実際の実装では、ランタイムから送信されたComponentデータを使用します
                        bevy::log::debug!("Entity updated from runtime: {:?}", entity);
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}

/// シーン変更をランタイムに同期
pub fn sync_scene_changes(
    _communication: Res<EditorRuntimeCommunication>,
    _scene_manager: Res<crate::systems::scene_management::SceneManager>,
) {
    // シーンが変更された場合、ランタイムに通知
    // 実際の実装では、シーン変更を検出するイベントを使用します
}
