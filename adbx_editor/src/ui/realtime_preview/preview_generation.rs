use super::{PreviewContent, PreviewType, RealtimePreview};
use std::time::Instant;

/// プレビューを更新
pub fn update_preview(
    preview: &mut RealtimePreview,
    file_path: String,
    content: String,
    preview_type: PreviewType,
) {
    // スロットルチェック
    if let Some(last_update) = preview.last_update_time.get(&file_path) {
        if last_update.elapsed() < preview.update_throttle {
            return; // スロットル時間内の場合は更新をスキップ
        }
    }

    // プレビューコンテンツを生成
    let (rendered_content, error) = match preview_type {
        PreviewType::Code => {
            // コードの場合は、シンタックスハイライト済みのHTMLを生成
            // 構文エラーのチェック（簡易実装）
            let syntax_error = check_syntax_errors(&content, preview_type);
            if let Some(err) = syntax_error {
                (None, Some(err))
            } else {
                (
                    Some(format!("<pre><code>{}</code></pre>", html_escape(&content))),
                    None,
                )
            }
        }
        PreviewType::Html => {
            // HTMLの場合はそのまま
            (Some(content.clone()), None)
        }
        PreviewType::Image => {
            // 画像の場合は、base64エンコードされたデータURIを生成
            // 画像ファイルの読み込みは、実際のファイルシステムから行う必要がある
            (
                Some("Image preview: File must be loaded from filesystem".to_string()),
                None,
            )
        }
        PreviewType::Scene => {
            // シーンの場合は、シーン情報を表示
            let entity_count = content.lines().filter(|l| l.contains("Entity")).count();
            (
                Some(format!(
                    "Scene Preview:\n- {} entities detected\n- File: {}",
                    entity_count, file_path
                )),
                None,
            )
        }
        PreviewType::Script => {
            // スクリプトの場合は、実行結果を表示
            // Luaスクリプトの構文チェック
            let syntax_error = check_syntax_errors(&content, preview_type);
            if let Some(err) = syntax_error {
                (None, Some(err))
            } else {
                (Some("Script syntax is valid. Execution result will be shown here when script runs.".to_string()), None)
            }
        }
    };

    preview.preview_content.insert(
        file_path.clone(),
        PreviewContent {
            content,
            rendered_content,
            preview_type,
            error,
        },
    );

    preview.last_update_time.insert(file_path, Instant::now());
}

/// HTMLエスケープ
fn html_escape(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// 構文エラーをチェック
fn check_syntax_errors(content: &str, preview_type: PreviewType) -> Option<String> {
    match preview_type {
        PreviewType::Script => {
            // Luaスクリプトの構文チェック
            use mlua::Lua;
            let lua = Lua::new();
            match lua.load(content).exec() {
                Ok(_) => None,
                Err(e) => Some(format!("Lua syntax error: {}", e)),
            }
        }
        PreviewType::Code => {
            // 基本的な構文チェック（括弧の対応など）
            let mut paren_count = 0;
            let mut brace_count = 0;
            let mut bracket_count = 0;

            for ch in content.chars() {
                match ch {
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    '{' => brace_count += 1,
                    '}' => brace_count -= 1,
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => {}
                }

                if paren_count < 0 || brace_count < 0 || bracket_count < 0 {
                    return Some("Mismatched brackets detected".to_string());
                }
            }

            if paren_count != 0 {
                Some(format!("Unmatched parentheses: {} unclosed", paren_count))
            } else if brace_count != 0 {
                Some(format!("Unmatched braces: {} unclosed", brace_count))
            } else if bracket_count != 0 {
                Some(format!("Unmatched brackets: {} unclosed", bracket_count))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// ファイルパスからプレビュータイプを検出
pub fn detect_preview_type(file_path: &str) -> PreviewType {
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "html" | "htm" => PreviewType::Html,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" => PreviewType::Image,
        "scene" | "scn" => PreviewType::Scene,
        "lua" => PreviewType::Script,
        _ => PreviewType::Code,
    }
}
