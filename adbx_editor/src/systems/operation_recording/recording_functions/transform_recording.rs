use super::super::{OperationContext, OperationRecorder, OperationType};
use bevy::prelude::*;

/// Transform変更を記録
pub fn record_transform_changed(
    recorder: &mut OperationRecorder,
    entity: Entity,
    _old_transform: Option<Transform>,
    new_transform: Transform,
) {
    recorder.record_operation(
        OperationType::TransformChanged,
        OperationContext {
            entity_id: Some(entity.index()),
            component_type: Some("Transform".to_string()),
            panel_name: Some("InspectorPanel".to_string()),
            file_path: None,
            user_intent: Some("Modify entity transform".to_string()),
        },
        serde_json::json!({
            "translation": {
                "x": new_transform.translation.x,
                "y": new_transform.translation.y,
                "z": new_transform.translation.z,
            },
            "rotation": {
                "x": new_transform.rotation.x,
                "y": new_transform.rotation.y,
                "z": new_transform.rotation.z,
                "w": new_transform.rotation.w,
            },
            "scale": {
                "x": new_transform.scale.x,
                "y": new_transform.scale.y,
                "z": new_transform.scale.z,
            },
        }),
    );
}
