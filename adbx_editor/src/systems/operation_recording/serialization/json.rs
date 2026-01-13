use super::super::OperationRecorder;

/// 操作履歴をJSON形式で取得（AI APIに送信する形式）
pub fn get_history_json(
    recorder: &OperationRecorder,
    limit: Option<usize>,
) -> Result<String, String> {
    let limit = limit.unwrap_or(recorder.operations.len());
    let operations: Vec<&super::super::RecordedOperation> =
        recorder.operations.iter().rev().take(limit).collect();

    serde_json::to_string_pretty(&operations.iter().rev().collect::<Vec<_>>())
        .map_err(|e| format!("Failed to serialize operations: {}", e))
}
