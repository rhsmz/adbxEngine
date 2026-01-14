use super::super::{OperationContext, OperationRecorder, OperationType};
use bevy::prelude::*;

/// コード生成を記録
#[allow(dead_code)]
pub fn record_code_generated(
    mut recorder: ResMut<OperationRecorder>,
    prompt: String,
    generated_code: String,
    file_path: Option<String>,
) {
    recorder.record_operation(
        OperationType::CodeGenerated,
        OperationContext {
            entity_id: None,
            component_type: None,
            panel_name: Some("CodeEditorPanel".to_string()),
            file_path: file_path.clone(),
            user_intent: Some(format!("Generate code from prompt: {}", prompt)),
        },
        serde_json::json!({
            "prompt": prompt,
            "generated_code_length": generated_code.len(),
            "file_path": file_path,
        }),
    );
}
