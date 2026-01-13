use std::path::PathBuf;
use super::OperationRecorder;
use super::serialization::{get_history_json, get_ai_readable_history, get_ai_context};

/// 操作記録をファイルにエクスポート（JSON形式）
pub fn export_operations_to_json(
    recorder: &OperationRecorder,
    file_path: PathBuf,
) -> Result<(), String> {
    let json_content = get_history_json(recorder, None)?;
    std::fs::write(&file_path, json_content)
        .map_err(|e| format!("Failed to write operations to file: {}", e))?;
    Ok(())
}

/// 操作記録をファイルにエクスポート（AI読み取り可能形式）
pub fn export_operations_to_text(
    recorder: &OperationRecorder,
    file_path: PathBuf,
    limit: Option<usize>,
) -> Result<(), String> {
    let text_content = get_ai_readable_history(recorder, limit);
    std::fs::write(&file_path, text_content)
        .map_err(|e| format!("Failed to write operations to file: {}", e))?;
    Ok(())
}

/// 操作記録をファイルにエクスポート（AIコンテキスト形式）
pub fn export_operations_to_markdown(
    recorder: &OperationRecorder,
    file_path: PathBuf,
    limit: Option<usize>,
) -> Result<(), String> {
    let markdown_content = get_ai_context(recorder, limit);
    std::fs::write(&file_path, markdown_content)
        .map_err(|e| format!("Failed to write operations to file: {}", e))?;
    Ok(())
}
