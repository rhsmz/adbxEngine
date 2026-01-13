#[cfg(feature = "ai")]
use std::sync::mpsc;
#[cfg(feature = "ai")]
use crate::ui::code_editor::{AiRequest, AiResponse, AiProvider};
#[cfg(feature = "ai")]
use super::openai_client::generate_with_openai_async;
#[cfg(feature = "ai")]
use super::claude_client::generate_with_claude_async;

/// AIリクエストを非同期で処理
#[cfg(feature = "ai")]
pub fn process_ai_request_async(
    request_id: usize,
    client: reqwest::Client,
    api_key: String,
    provider: AiProvider,
    request: AiRequest,
    sender: mpsc::Sender<(usize, AiResponse)>,
) {
    // tokio::spawnを使用して非同期タスクを開始
    tokio::spawn(async move {
        bevy::log::info!("Starting AI request {} (provider: {:?})", request_id, provider);
        
        let response = match provider {
            AiProvider::OpenAI => {
                bevy::log::info!("Calling OpenAI API for request {}", request_id);
                generate_with_openai_async(client, api_key, &request).await
            }
            AiProvider::Claude => {
                bevy::log::info!("Calling Claude API for request {}", request_id);
                generate_with_claude_async(client, api_key, &request).await
            }
        };
        
        // 結果をチャネル経由で送信
        if let Err(e) = sender.send((request_id, response)) {
            bevy::log::error!("Failed to send AI response for request {}: {}", request_id, e);
        } else {
            bevy::log::info!("AI request {} completed", request_id);
        }
    });
}
