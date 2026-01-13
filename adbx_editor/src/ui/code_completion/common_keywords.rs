use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// 共通のキーワード
pub fn get_common_keywords(prefix: &str) -> Vec<CompletionCandidate> {
    let keywords = vec![
        "if", "else", "for", "while", "return", "true", "false", "null", "undefined",
    ];
    
    keywords
        .iter()
        .filter(|kw| kw.to_lowercase().starts_with(&prefix.to_lowercase()))
        .map(|kw| CompletionCandidate {
            label: kw.to_string(),
            insert_text: kw.to_string(),
            kind: CompletionKind::Keyword,
            detail: None,
        })
        .collect()
}
