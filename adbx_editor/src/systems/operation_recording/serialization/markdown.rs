use super::super::{OperationRecorder, RecordedOperation};

/// 操作履歴をAI API用のコンテキスト形式で取得
pub fn get_ai_context(recorder: &OperationRecorder, limit: Option<usize>) -> String {
    let limit = limit.unwrap_or(20); // デフォルトは最新20件
    let operations: Vec<&RecordedOperation> =
        recorder.operations.iter().rev().take(limit).collect();

    let mut context = String::new();
    context.push_str("## Editor Operation History\n\n");
    context.push_str("The following operations were performed in the editor:\n\n");

    for op in operations.iter().rev() {
        context.push_str(&format!("### Operation: {:?}\n", op.operation_type));

        if let Some(ref user_intent) = op.context.user_intent {
            context.push_str(&format!("**User Intent**: {}\n", user_intent));
        }

        if let Some(entity_id) = op.context.entity_id {
            context.push_str(&format!("**Entity ID**: {}\n", entity_id));
        }

        if let Some(ref component_type) = op.context.component_type {
            context.push_str(&format!("**Component Type**: {}\n", component_type));
        }

        if let Some(ref file_path) = op.context.file_path {
            context.push_str(&format!("**File Path**: {}\n", file_path));
        }

        if !op.details.is_null() {
            context.push_str("**Details**:\n```json\n");
            if let Ok(details_str) = serde_json::to_string_pretty(&op.details) {
                context.push_str(&details_str);
            }
            context.push_str("\n```\n");
        }

        context.push_str("\n---\n\n");
    }

    context
}
