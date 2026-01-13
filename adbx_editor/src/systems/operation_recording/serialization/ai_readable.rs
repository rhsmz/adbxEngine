use super::super::{OperationRecorder, RecordedOperation};

/// 操作履歴をAI理解可能な形式で取得（改善版）
pub fn get_ai_readable_history(recorder: &OperationRecorder, limit: Option<usize>) -> String {
    let limit = limit.unwrap_or(recorder.operations.len());
    let operations: Vec<&RecordedOperation> = recorder.operations
        .iter()
        .rev()
        .take(limit)
        .collect();
    
    let mut history = String::new();
    history.push_str("Recent Editor Operations:\n\n");
    
    for op in operations.iter().rev() {
        let operation_desc = match &op.operation_type {
            super::super::OperationType::EntityCreated => "Created entity",
            super::super::OperationType::EntityDeleted => "Deleted entity",
            super::super::OperationType::EntitySelected => "Selected entity",
            super::super::OperationType::ComponentAdded => "Added component",
            super::super::OperationType::ComponentRemoved => "Removed component",
            super::super::OperationType::ComponentModified => "Modified component",
            super::super::OperationType::TransformChanged => "Changed transform",
            super::super::OperationType::SceneSaved => "Saved scene",
            super::super::OperationType::SceneLoaded => "Loaded scene",
            super::super::OperationType::AssetImported => "Imported asset",
            super::super::OperationType::AssetExported => "Exported asset",
            super::super::OperationType::ScriptAttached => "Attached script",
            super::super::OperationType::ScriptModified => "Modified script",
            super::super::OperationType::CodeGenerated => "Generated code",
            super::super::OperationType::CodeRefactored => "Refactored code",
            super::super::OperationType::PanelMoved => "Moved panel",
            super::super::OperationType::PanelResized => "Resized panel",
            super::super::OperationType::SettingsChanged => "Changed settings",
        };
        
        history.push_str(&format!("- {}: ", operation_desc));
        
        if let Some(entity_id) = op.context.entity_id {
            history.push_str(&format!("Entity {}; ", entity_id));
        }
        
        if let Some(ref component_type) = op.context.component_type {
            history.push_str(&format!("Component {}; ", component_type));
        }
        
        if let Some(ref panel_name) = op.context.panel_name {
            history.push_str(&format!("Panel {}; ", panel_name));
        }
        
        if let Some(ref file_path) = op.context.file_path {
            history.push_str(&format!("File {}; ", file_path));
        }
        
        if let Some(ref user_intent) = op.context.user_intent {
            history.push_str(&format!("Intent: {}; ", user_intent));
        }
        
        // 詳細情報を追加（JSON形式から読みやすい形式に変換）
        if !op.details.is_null() {
            match &op.operation_type {
                super::super::OperationType::TransformChanged => {
                    if let Some(translation) = op.details.get("translation") {
                        if let (Some(x), Some(y), Some(z)) = (
                            translation.get("x").and_then(|v| v.as_f64()),
                            translation.get("y").and_then(|v| v.as_f64()),
                            translation.get("z").and_then(|v| v.as_f64()),
                        ) {
                            history.push_str(&format!("Translation: ({:.2}, {:.2}, {:.2}); ", x, y, z));
                        }
                    }
                    if let Some(rotation) = op.details.get("rotation") {
                        if let (Some(x), Some(y), Some(z), Some(w)) = (
                            rotation.get("x").and_then(|v| v.as_f64()),
                            rotation.get("y").and_then(|v| v.as_f64()),
                            rotation.get("z").and_then(|v| v.as_f64()),
                            rotation.get("w").and_then(|v| v.as_f64()),
                        ) {
                            history.push_str(&format!("Rotation: ({:.2}, {:.2}, {:.2}, {:.2}); ", x, y, z, w));
                        }
                    }
                    if let Some(scale) = op.details.get("scale") {
                        if let (Some(x), Some(y), Some(z)) = (
                            scale.get("x").and_then(|v| v.as_f64()),
                            scale.get("y").and_then(|v| v.as_f64()),
                            scale.get("z").and_then(|v| v.as_f64()),
                        ) {
                            history.push_str(&format!("Scale: ({:.2}, {:.2}, {:.2}); ", x, y, z));
                        }
                    }
                }
                super::super::OperationType::CodeGenerated => {
                    if let Some(prompt) = op.details.get("prompt").and_then(|v| v.as_str()) {
                        history.push_str(&format!("Prompt: \"{}\"; ", prompt));
                    }
                    if let Some(code_length) = op.details.get("generated_code_length").and_then(|v| v.as_u64()) {
                        history.push_str(&format!("Generated code length: {} characters; ", code_length));
                    }
                }
                super::super::OperationType::PanelMoved => {
                    if let Some(old_pos) = op.details.get("old_position").and_then(|v| v.as_str()) {
                        history.push_str(&format!("From: {}; ", old_pos));
                    }
                    if let Some(new_pos) = op.details.get("new_position").and_then(|v| v.as_str()) {
                        history.push_str(&format!("To: {}; ", new_pos));
                    }
                }
                super::super::OperationType::SceneSaved | super::super::OperationType::SceneLoaded => {
                    if let Some(scene_name) = op.details.get("scene_name").and_then(|v| v.as_str()) {
                        history.push_str(&format!("Scene: {}; ", scene_name));
                    }
                }
                _ => {
                    // その他の操作タイプは、詳細情報を文字列として表示
                    if let Some(details_str) = op.details.as_str() {
                        if !details_str.is_empty() {
                            history.push_str(&format!("Details: {}; ", details_str));
                        }
                    } else if let Ok(details_str) = serde_json::to_string_pretty(&op.details) {
                        if details_str != "null" && details_str != "{}" {
                            history.push_str(&format!("Details: {}; ", details_str));
                        }
                    }
                }
            }
        }
        
        history.push('\n');
    }
    
    history
}
