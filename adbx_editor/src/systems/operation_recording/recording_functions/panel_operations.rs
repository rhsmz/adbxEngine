use bevy::prelude::*;
use super::super::{OperationRecorder, OperationType, OperationContext};

/// パネル移動を記録
pub fn record_panel_moved(
    mut recorder: ResMut<OperationRecorder>,
    panel_name: String,
    old_position: String,
    new_position: String,
) {
    recorder.record_operation(
        OperationType::PanelMoved,
        OperationContext {
            entity_id: None,
            component_type: None,
            panel_name: Some(panel_name),
            file_path: None,
            user_intent: Some(format!("Move panel from {} to {}", old_position, new_position)),
        },
        serde_json::json!({
            "old_position": old_position,
            "new_position": new_position,
        }),
    );
}

/// パネルリサイズを記録
pub fn record_panel_resized(
    mut recorder: ResMut<OperationRecorder>,
    panel_name: String,
    old_size: (f32, f32),
    new_size: (f32, f32),
) {
    recorder.record_operation(
        OperationType::PanelResized,
        OperationContext {
            entity_id: None,
            component_type: None,
            panel_name: Some(panel_name),
            file_path: None,
            user_intent: Some(format!("Resize panel from ({:.1}, {:.1}) to ({:.1}, {:.1})", 
                old_size.0, old_size.1, new_size.0, new_size.1)),
        },
        serde_json::json!({
            "old_size": {
                "width": old_size.0,
                "height": old_size.1,
            },
            "new_size": {
                "width": new_size.0,
                "height": new_size.1,
            },
        }),
    );
}
