use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// コンテンツから識別子（関数名、変数名）を抽出
pub fn extract_identifiers_from_content(
    content: &str,
    prefix: &str,
    _cursor_position: usize,
) -> Vec<CompletionCandidate> {
    let mut candidates = Vec::new();
    
    // 簡単な正規表現で識別子を抽出（関数定義、変数定義など）
    // 実際の実装では、より高度なパーサーを使用することを推奨
    let identifier_pattern = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
    
    for cap in identifier_pattern.find_iter(content) {
        let identifier = cap.as_str();
        
        // プレフィックスに一致するかチェック
        if identifier.to_lowercase().starts_with(&prefix.to_lowercase()) {
            // キーワードは除外
            if !is_keyword(identifier) {
                candidates.push(CompletionCandidate {
                    label: identifier.to_string(),
                    insert_text: identifier.to_string(),
                    kind: CompletionKind::Variable,
                    detail: None,
                });
            }
        }
    }
    
    candidates
}

/// キーワードかどうかを判定
fn is_keyword(word: &str) -> bool {
    let keywords = vec![
        "if", "else", "for", "while", "return", "true", "false", "null", "undefined",
        "fn", "let", "mut", "const", "static", "pub", "use", "mod", "struct", "enum",
        "function", "var", "let", "const", "class", "def", "import", "from",
    ];
    
    keywords.contains(&word)
}
