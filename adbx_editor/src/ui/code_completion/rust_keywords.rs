use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// Rustのキーワード
pub fn get_rust_keywords(prefix: &str) -> Vec<CompletionCandidate> {
    let keywords = vec![
        "fn", "let", "mut", "const", "static", "pub", "use", "mod", "struct", "enum",
        "impl", "trait", "where", "if", "else", "match", "for", "while", "loop", "break",
        "continue", "return", "self", "Self", "super", "crate", "async", "await", "move",
        "ref", "dyn", "unsafe", "extern", "type", "as", "in", "true", "false", "None",
        "Some", "Ok", "Err", "Result", "Option", "Box", "Vec", "String", "str", "u8",
        "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
        "bool", "char", "usize", "isize",
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

/// Rustの関数
pub fn get_rust_functions(prefix: &str) -> Vec<CompletionCandidate> {
    let functions = vec![
        ("println!", "println!(\"{}\", value)", "fn println!(fmt: &str, args: ...) -> ()", "Prints to stdout with a newline"),
        ("print!", "print!(\"{}\", value)", "fn print!(fmt: &str, args: ...) -> ()", "Prints to stdout"),
        ("eprintln!", "eprintln!(\"{}\", value)", "fn eprintln!(fmt: &str, args: ...) -> ()", "Prints to stderr with a newline"),
        ("format!", "format!(\"{}\", value)", "fn format!(fmt: &str, args: ...) -> String", "Formats a string"),
        ("vec!", "vec![value1, value2]", "macro vec![...] -> Vec<T>", "Creates a vector"),
        ("String::new", "String::new()", "fn new() -> String", "Creates a new empty String"),
        ("String::from", "String::from(\"text\")", "fn from(s: &str) -> String", "Creates a String from a string slice"),
        ("Vec::new", "Vec::new()", "fn new() -> Vec<T>", "Creates a new empty vector"),
        ("Vec::with_capacity", "Vec::with_capacity(size)", "fn with_capacity(capacity: usize) -> Vec<T>", "Creates a vector with the given capacity"),
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
