use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// JavaScriptのキーワード
pub fn get_javascript_keywords(prefix: &str) -> Vec<CompletionCandidate> {
    let keywords = vec![
        "function",
        "var",
        "let",
        "const",
        "if",
        "else",
        "for",
        "while",
        "do",
        "switch",
        "case",
        "break",
        "continue",
        "return",
        "try",
        "catch",
        "finally",
        "throw",
        "new",
        "this",
        "super",
        "class",
        "extends",
        "static",
        "async",
        "await",
        "yield",
        "import",
        "export",
        "default",
        "from",
        "as",
        "typeof",
        "instanceof",
        "in",
        "true",
        "false",
        "null",
        "undefined",
        "NaN",
        "Infinity",
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

/// JavaScriptの関数
pub fn get_javascript_functions(prefix: &str) -> Vec<CompletionCandidate> {
    let functions = vec![
        (
            "console.log",
            "console.log(...args)",
            "function log(...args: any[]): void",
            "Prints values to console",
        ),
        (
            "console.error",
            "console.error(...args)",
            "function error(...args: any[]): void",
            "Prints error to console",
        ),
        (
            "console.warn",
            "console.warn(...args)",
            "function warn(...args: any[]): void",
            "Prints warning to console",
        ),
        (
            "Array.isArray",
            "Array.isArray(value)",
            "function isArray(value: any): boolean",
            "Checks if value is an array",
        ),
        (
            "Object.keys",
            "Object.keys(object)",
            "function keys(object: object): string[]",
            "Returns array of object keys",
        ),
        (
            "Object.values",
            "Object.values(object)",
            "function values(object: object): any[]",
            "Returns array of object values",
        ),
        (
            "JSON.stringify",
            "JSON.stringify(value)",
            "function stringify(value: any): string",
            "Converts value to JSON string",
        ),
        (
            "JSON.parse",
            "JSON.parse(text)",
            "function parse(text: string): any",
            "Parses JSON string",
        ),
        (
            "parseInt",
            "parseInt(string, radix)",
            "function parseInt(string: string, radix?: number): number",
            "Parses string to integer",
        ),
        (
            "parseFloat",
            "parseFloat(string)",
            "function parseFloat(string: string): number",
            "Parses string to float",
        ),
    ];

    functions
        .iter()
        .filter(|(name, _, _, _)| name.to_lowercase().starts_with(&prefix.to_lowercase()))
        .map(|(name, insert, signature, doc)| CompletionCandidate {
            label: name.to_string(),
            insert_text: insert.to_string(),
            kind: CompletionKind::Function,
            detail: Some(format!("{}\n{}", signature, doc)),
        })
        .collect()
}
