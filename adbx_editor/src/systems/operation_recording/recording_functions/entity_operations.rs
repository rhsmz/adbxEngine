use super::super::{OperationContext, OperationRecorder, OperationType};
use bevy::prelude::*;

/// 操作を記録するヘルパー関数
pub fn record_entity_created(
    mut recorder: ResMut<OperationRecorder>,
    entity: Entity,
    entity_name: Option<String>,
) {
    recorder.record_operation(
        OperationType::EntityCreated,
        OperationContext {
            entity_id: Some(entity.index()),
            component_type: None,
            panel_name: None,
            file_path: None,
            user_intent: Some(format!(
                "Create new entity: {}",
                entity_name.as_deref().unwrap_or("Unnamed")
            )),
        },
        serde_json::json!({
            "entity_name": entity_name,
            "entity_index": entity.index(),
        }),
    );
}
