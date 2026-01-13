use crate::scene::SceneData;
use serde::{Deserialize, Serialize};

/// エディタ-ランタイム通信メッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EditorMessage {
    /// シーンの読み込み要求
    LoadScene { scene_path: String },
    /// シーンの保存要求
    SaveScene { scene: SceneData },
    /// Entityの選択通知
    SelectEntity { entity_id: u32 },
    /// Componentの更新通知
    UpdateComponent {
        entity_id: u32,
        component_data: crate::scene::ComponentData,
    },
    /// アセットの読み込み要求
    LoadAsset { asset_path: String },
    /// ホットリロード通知
    HotReload { asset_path: String },
    /// スクリプトの実行要求
    ExecuteScript { script_content: String },
    /// Entityへのスクリプトアタッチ要求
    AttachScript { entity_id: u32, script_path: String },
    /// Entityからスクリプトを削除
    DetachScript { entity_id: u32 },
}

/// ランタイム-エディタ通信メッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuntimeMessage {
    /// シーンの読み込み完了
    SceneLoaded { scene: SceneData },
    /// Entityの状態更新
    EntityUpdated { entity_id: u32 },
    /// エラー通知
    Error { message: String },
    /// ホットリロード完了
    HotReloaded { asset_path: String },
    /// スクリプト実行完了
    ScriptExecuted {
        success: bool,
        error_message: Option<String>,
    },
    /// スクリプトアタッチ完了
    ScriptAttached { entity_id: u32, script_path: String },
    /// スクリプト削除完了
    ScriptDetached { entity_id: u32 },
}
