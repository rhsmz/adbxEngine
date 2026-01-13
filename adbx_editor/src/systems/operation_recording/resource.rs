use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// エディタ操作の記録システム
#[derive(Resource)]
pub struct OperationRecorder {
    pub operations: VecDeque<RecordedOperation>,
    pub max_history: usize,
    pub is_recording: bool,
}

/// 記録された操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedOperation {
    pub timestamp: u64,
    pub operation_type: OperationType,
    pub context: OperationContext,
    pub details: serde_json::Value,
}

/// 操作の種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    EntityCreated,
    EntityDeleted,
    EntitySelected,
    ComponentAdded,
    ComponentRemoved,
    ComponentModified,
    TransformChanged,
    SceneSaved,
    SceneLoaded,
    AssetImported,
    AssetExported,
    ScriptAttached,
    ScriptModified,
    CodeGenerated,
    CodeRefactored,
    PanelMoved,
    PanelResized,
    SettingsChanged,
}

/// 操作のコンテキスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationContext {
    pub entity_id: Option<u32>,
    pub component_type: Option<String>,
    pub panel_name: Option<String>,
    pub file_path: Option<String>,
    pub user_intent: Option<String>, // ユーザーの意図を推測
}

impl OperationRecorder {
    pub fn new(max_history: usize) -> Self {
        Self {
            operations: VecDeque::with_capacity(max_history),
            max_history,
            is_recording: true,
        }
    }

    /// 操作を記録
    pub fn record_operation(
        &mut self,
        operation_type: OperationType,
        context: OperationContext,
        details: serde_json::Value,
    ) {
        if !self.is_recording {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let operation = RecordedOperation {
            timestamp,
            operation_type,
            context,
            details,
        };

        self.operations.push_back(operation);

        // 最大履歴数を超えた場合、古い操作を削除
        while self.operations.len() > self.max_history {
            self.operations.pop_front();
        }
    }

    /// 操作履歴をクリア
    pub fn clear_history(&mut self) {
        self.operations.clear();
    }
}

impl Default for OperationRecorder {
    fn default() -> Self {
        Self::new(1000)
    }
}
