use crate::ui::code_editor::CompletionCandidate;
use super::rust_keywords::{get_rust_keywords, get_rust_functions};
use super::lua_keywords::{get_lua_keywords, get_lua_functions};
use super::python_keywords::{get_python_keywords, get_python_functions};
use super::javascript_keywords::{get_javascript_keywords, get_javascript_functions};
use super::common_keywords::get_common_keywords;
use super::identifier_extraction::extract_identifiers_from_content;

/// コード補完の候補を生成
pub fn get_completion_candidates(
    prefix: &str,
    language: Option<&str>,
    content: &str,
    cursor_position: usize,
) -> Vec<CompletionCandidate> {
    let mut candidates = Vec::new();
    
    // 言語に応じたキーワードを追加
    if let Some(lang) = language {
        match lang {
            "Rust" => {
                candidates.extend(get_rust_keywords(prefix));
                candidates.extend(get_rust_functions(prefix));
            }
            "Lua" => {
                candidates.extend(get_lua_keywords(prefix));
                candidates.extend(get_lua_functions(prefix));
            }
            "Python" => {
                candidates.extend(get_python_keywords(prefix));
                candidates.extend(get_python_functions(prefix));
            }
            "JavaScript" | "TypeScript" => {
                candidates.extend(get_javascript_keywords(prefix));
                candidates.extend(get_javascript_functions(prefix));
            }
            _ => {
                // 汎用的なキーワード
                candidates.extend(get_common_keywords(prefix));
            }
        }
    } else {
        candidates.extend(get_common_keywords(prefix));
    }
    
    // 現在のファイルから関数名や変数名を抽出
    candidates.extend(extract_identifiers_from_content(content, prefix, cursor_position));
    
    // プレフィックスでフィルタリング
    candidates.retain(|c| c.label.to_lowercase().starts_with(&prefix.to_lowercase()));
    
    // 重複を除去
    candidates.sort_by(|a, b| a.label.cmp(&b.label));
    candidates.dedup_by(|a, b| a.label == b.label);
    
    candidates
}
