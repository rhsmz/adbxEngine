use bevy::prelude::*;
use super::resource::{AiRequest, AiRequestType};

/// AIによるコード生成（非同期処理のためのリクエストキューに追加）
pub fn request_ai_code_generation(
    mut code_editor: ResMut<crate::ui::code_editor::resource::CodeEditor>,
    prompt: String,
    context: Option<String>,
) {
    code_editor.ai_requests.push_back(AiRequest {
        prompt: prompt.clone(),
        context: context.clone(),
        request_type: AiRequestType::Generate,
    });
}

/// AIによるコード補完（非同期処理のためのリクエストキューに追加）
pub fn request_ai_code_completion(
    mut code_editor: ResMut<crate::ui::code_editor::resource::CodeEditor>,
    prompt: String,
    context: Option<String>,
) {
    code_editor.ai_requests.push_back(AiRequest {
        prompt,
        context,
        request_type: AiRequestType::Complete,
    });
}

/// AIによるコードリファクタリング（非同期処理のためのリクエストキューに追加）
pub fn request_ai_code_refactor(
    mut code_editor: ResMut<crate::ui::code_editor::resource::CodeEditor>,
    prompt: String,
    context: Option<String>,
) {
    code_editor.ai_requests.push_back(AiRequest {
        prompt: prompt.clone(),
        context: context.clone(),
        request_type: AiRequestType::Refactor,
    });
}
