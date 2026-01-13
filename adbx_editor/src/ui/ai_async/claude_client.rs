#[cfg(feature = "ai")]
use crate::ui::code_editor::{AiRequest, AiResponse};

/// Claude APIを使用してコードを生成（非同期版）
#[cfg(feature = "ai")]
pub async fn generate_with_claude_async(
    client: reqwest::Client,
    api_key: String,
    request: &AiRequest,
) -> AiResponse {
    use serde_json::json;

    // Claude APIのエンドポイント
    let url = "https://api.anthropic.com/v1/messages";

    // リクエストボディを構築
    let mut messages = vec![json!({
        "role": "user",
        "content": request.prompt
    })];

    // コンテキストがある場合は追加
    if let Some(ref context) = request.context {
        messages[0]["content"] = json!(format!("{}\n\nContext:\n{}", request.prompt, context));
    }

    let request_body = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 4096,
        "messages": messages
    });

    // 非同期でリクエストを送信
    match client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // Claude APIのレスポンスからコンテンツを抽出
                        if let Some(content) = json["content"]
                            .as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|item| item["text"].as_str())
                        {
                            AiResponse {
                                content: content.to_string(),
                                success: true,
                                error: None,
                            }
                        } else {
                            AiResponse {
                                content: String::new(),
                                success: false,
                                error: Some("Failed to parse Claude API response".to_string()),
                            }
                        }
                    }
                    Err(e) => AiResponse {
                        content: String::new(),
                        success: false,
                        error: Some(format!("Failed to parse response: {}", e)),
                    },
                }
            } else {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                AiResponse {
                    content: String::new(),
                    success: false,
                    error: Some(format!("Claude API error ({}): {}", status, error_text)),
                }
            }
        }
        Err(e) => AiResponse {
            content: String::new(),
            success: false,
            error: Some(format!("Request failed: {}", e)),
        },
    }
}
