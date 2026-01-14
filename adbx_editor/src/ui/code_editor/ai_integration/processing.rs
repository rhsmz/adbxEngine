use super::resource::{AiIntegration, AiResponse};
use bevy::prelude::*;

/// AIリクエストを処理するシステム（非同期処理）
pub fn process_pending_ai_code_generation_requests(
    mut code_editor: ResMut<crate::ui::code_editor::resource::CodeEditor>,
    _ai_integration: ResMut<AiIntegration>,
    _operation_recorder: Option<ResMut<crate::systems::operation_recording::OperationRecorder>>,
    #[cfg(feature = "ai")] ai_async: ResMut<crate::ui::ai_async::AiAsyncProcessor>,
) {
    #[cfg(feature = "ai")]
    {
        // 完了したリクエストをチェック
        if let Some(ref receiver_mutex) = ai_async.response_receiver {
            if let Ok(receiver) = receiver_mutex.lock() {
                while let Ok((request_id, response)) = receiver.try_recv() {
                    // リクエストを完了としてマーク
                    if let Some(status) = ai_integration.pending_requests.get_mut(&request_id) {
                        if response.success {
                            *status = AiRequestStatus::Completed(code_editor.ai_responses.len());
                            code_editor.ai_progress =
                                Some(format!("AI request {} completed successfully", request_id));
                        } else {
                            *status = AiRequestStatus::Failed(
                                response
                                    .error
                                    .clone()
                                    .unwrap_or_else(|| "Unknown error".to_string()),
                            );
                            code_editor.ai_progress = Some(format!(
                                "AI request {} failed: {}",
                                request_id,
                                response
                                    .error
                                    .as_ref()
                                    .unwrap_or(&"Unknown error".to_string())
                            ));
                        }
                    }

                    // レスポンスを追加
                    code_editor.ai_responses.push_back(response);
                }
            }
        }

        // 進行中のリクエスト数を表示
        let processing_count = ai_integration
            .pending_requests
            .values()
            .filter(|status| {
                matches!(
                    status,
                    AiRequestStatus::Pending | AiRequestStatus::Processing
                )
            })
            .count();

        if processing_count > 0 && code_editor.ai_progress.is_none() {
            code_editor.ai_progress =
                Some(format!("Processing {} AI request(s)...", processing_count));
        }

        // 新しいリクエストを処理
        while let Some(request) = code_editor.ai_requests.pop_front() {
            if let Some(ref client) = ai_integration.client {
                if let Some(ref api_key) = ai_integration.api_key {
                    // リクエストIDを割り当て
                    let request_id = ai_integration.next_request_id;
                    ai_integration.next_request_id += 1;

                    // リクエストを待機中としてマーク
                    ai_integration
                        .pending_requests
                        .insert(request_id, AiRequestStatus::Pending);

                    // 進行状況を表示
                    code_editor.ai_progress =
                        Some(format!("AI request {} is processing...", request_id));

                    // 操作を記録
                    if let Some(ref mut recorder) = _operation_recorder {
                        let current_file = code_editor.current_file.clone();
                        let prompt = request.prompt.clone();
                        crate::systems::operation_recording::record_code_generated(
                            recorder,
                            prompt,
                            String::new(), // 生成されたコードは後で追加される
                            current_file,
                        );
                    }

                    // 非同期処理を開始
                    let client_clone = client.clone();
                    let api_key_clone = api_key.clone();
                    let provider = ai_integration.provider;
                    let request_clone = request.clone();

                    if let Some(ref sender) = ai_async.response_sender {
                        let sender_clone = sender.clone();

                        // tokio::spawnを使用して非同期処理を開始
                        tokio::spawn(async move {
                            crate::ui::ai_async::process_ai_request_async(
                                request_id,
                                client_clone,
                                api_key_clone,
                                provider,
                                request_clone,
                                sender_clone,
                            );
                        });
                    }

                    // リクエストを処理中としてマーク
                    ai_integration
                        .pending_requests
                        .insert(request_id, AiRequestStatus::Processing);
                } else {
                    code_editor.ai_responses.push_back(AiResponse {
                        content: String::new(),
                        success: false,
                        error: Some("API key not set".to_string()),
                    });
                }
            } else {
                code_editor.ai_responses.push_back(AiResponse {
                    content: String::new(),
                    success: false,
                    error: Some("AI client not initialized".to_string()),
                });
            }
        }
    }
    #[cfg(not(feature = "ai"))]
    {
        while let Some(_request) = code_editor.ai_requests.pop_front() {
            code_editor.ai_responses.push_back(AiResponse {
                content: String::new(),
                success: false,
                error: Some(
                    "AI feature not enabled. Build with --features ai to enable.".to_string(),
                ),
            });
        }
    }
}

/// AIレスポンスを処理するシステム
pub fn handle_ai_code_generation_responses(
    mut code_editor: ResMut<crate::ui::code_editor::resource::CodeEditor>,
) {
    while let Some(response) = code_editor.ai_responses.pop_front() {
        if response.success {
            // レスポンスの内容をコードエディタに適用
            code_editor.content = response.content;
        } else {
            // エラーをログに記録
            bevy::log::error!("AI request failed: {:?}", response.error);
        }
    }
}
