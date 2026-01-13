use super::message_sending::send_to_runtime;
use super::EditorRuntimeCommunication;
use bevy::prelude::*;

/// エディタからランタイムにEntity選択を通知
pub fn notify_entity_selection(
    communication: &EditorRuntimeCommunication,
    selected_entities: &[Entity],
) {
    // 簡易実装：最初の選択されたエンティティのIDを送信
    // 注意: Entity::index()は実行時に変わる可能性があるため、実際の実装では
    // エンティティIDマッピングを使用する必要があります
    if let Some(&entity) = selected_entities.first() {
        if let Err(e) = send_to_runtime(
            communication,
            adbx_shared::EditorMessage::SelectEntity {
                entity_id: entity.index() as u32,
            },
        ) {
            bevy::log::warn!("Failed to notify entity selection: {}", e);
        }
    }
}

/// エディタからランタイムにComponent更新を通知
pub fn notify_component_update(
    communication: &EditorRuntimeCommunication,
    entity: Entity,
    component_data: adbx_shared::scene::ComponentData,
) {
    if let Err(e) = send_to_runtime(
        communication,
        adbx_shared::EditorMessage::UpdateComponent {
            entity_id: entity.index() as u32,
            component_data,
        },
    ) {
        bevy::log::warn!("Failed to notify component update: {}", e);
    }
}

/// エディタからランタイムにホットリロードを通知
pub fn notify_hot_reload(
    communication: &EditorRuntimeCommunication,
    asset_path: String,
) -> Result<(), String> {
    send_to_runtime(
        communication,
        adbx_shared::EditorMessage::HotReload { asset_path },
    )
}
