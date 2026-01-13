use bevy::prelude::*;
use super::super::{OperationRecorder, OperationType, OperationContext};

/// シーン保存を記録
pub fn record_scene_saved(
    recorder: &mut ResMut<OperationRecorder>,
    scene_name: String,
    scene_path: String,
) {
    recorder.record_operation(
        OperationType::SceneSaved,
        OperationContext {
            entity_id: None,
            component_type: None,
            panel_name: None,
            file_path: Some(scene_path.clone()),
            user_intent: Some(format!("Save scene: {}", scene_name)),
        },
        serde_json::json!({
            "scene_name": scene_name,
            "scene_path": scene_path,
        }),
    );
}
