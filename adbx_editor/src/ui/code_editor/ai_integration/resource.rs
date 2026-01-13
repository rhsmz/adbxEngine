use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI統合のリソース
#[derive(Resource)]
pub struct AiIntegration {
    pub api_key: Option<String>,
    pub provider: AiProvider,
    #[cfg(feature = "ai")]
    pub client: Option<reqwest::Client>,
    #[cfg(not(feature = "ai"))]
    pub _client: Option<()>,
    pub pending_requests: HashMap<usize, AiRequestStatus>, // 進行中のリクエスト
    pub next_request_id: usize, // 次のリクエストID
}

/// AIリクエストの状態
#[derive(Debug, Clone)]
pub enum AiRequestStatus {
    Pending, // 待機中
    Processing, // 処理中
    Completed(usize), // 完了（レスポンスのインデックス）
    Failed(String), // 失敗（エラーメッセージ）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiProvider {
    OpenAI,
    Claude,
}

impl Default for AiIntegration {
    fn default() -> Self {
        Self {
            api_key: None,
            provider: AiProvider::OpenAI,
            #[cfg(feature = "ai")]
            client: Some(reqwest::Client::new()),
            #[cfg(not(feature = "ai"))]
            _client: Some(()),
            pending_requests: HashMap::new(),
            next_request_id: 0,
        }
    }
}

/// AIリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub prompt: String,
    pub context: Option<String>,
    pub request_type: AiRequestType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiRequestType {
    Generate,
    Complete,
    Refactor,
    Explain,
}

/// AIレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
}
