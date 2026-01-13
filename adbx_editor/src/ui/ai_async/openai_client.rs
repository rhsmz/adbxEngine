#[cfg(feature = "ai")]
use crate::ui::code_editor::{AiRequest, AiResponse, AiRequestType};

/// OpenAI APIを使用してコードを生成（非同期版）
#[cfg(feature = "ai")]
pub async fn generate_with_openai_async(
    client: reqwest::Client,
    api_key: String,
    request: &AiRequest,
) -> AiResponse {
    use serde_json::json;
    
    // OpenAI APIのエンドポイント
    let url = "https://api.openai.com/v1/chat/completions";
    
    // リクエストタイプに応じたシステムメッセージを構築
    let system_message = match request.request_type {
        AiRequestType::Generate => "You are a helpful coding assistant. Generate code based on the user's request.",
        AiRequestType::Complete => "You are a helpful coding assistant. Complete the code based on the user's request.",
        AiRequestType::Refactor => "You are a helpful coding assistant. Refactor the code based on the user's request.",
        AiRequestType::Explain => "You are a helpful coding assistant. Explain the code based on the user's request.",
    };
    
    // メッセージを構築
    let mut messages = vec![
        json!({
            "role": "system",
            "content": system_message
        }),
        json!({
            "role": "user",
            "content": request.prompt
        })
    ];
    
    // コンテキストがある場合は追加
    if let Some(ref context) = request.context {
        messages.push(json!({
            "role": "user",
            "content": format!("Context:\n{}", context)
        }));
    }
    
    // リクエストボディを構築
    let request_body = json!({
        "model": "gpt-4o",
        "messages": messages,
        "temperature": 0.7,
        "max_tokens": 4096
    });
    
    // 非同期でリクエストを送信
    match client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json_response) => {
                        // レスポンスからテキストを抽出
                        if let Some(choices) = json_response.get("choices").and_then(|c| c.as_array()) {
                            if let Some(first_choice) = choices.first() {
                                if let Some(message) = first_choice.get("message") {
                                    if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                                        return AiResponse {
                                            content: content.to_string(),
                                            success: true,
                                            error: None,
                                        };
                                    }
                                }
                            }
                        }
                        
                        AiResponse {
                            content: String::new(),
                            success: false,
                            error: Some("Failed to parse OpenAI response".to_string()),
                        }
                    }
                    Err(e) => AiResponse {
                        content: String::new(),
                        success: false,
                        error: Some(format!("Failed to parse OpenAI response: {}", e)),
                    },
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                AiResponse {
                    content: String::new(),
                    success: false,
                    error: Some(format!("OpenAI API error ({}): {}", status, error_text)),
                }
            }
        }
        Err(e) => AiResponse {
            content: String::new(),
            success: false,
            error: Some(format!("Failed to send request to OpenAI: {}", e)),
        },
    }
}
