use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// Luaのキーワード
pub fn get_lua_keywords(prefix: &str) -> Vec<CompletionCandidate> {
    let keywords = vec![
        "function", "end", "if", "then", "else", "elseif", "for", "while", "do", "repeat",
        "until", "break", "return", "local", "nil", "true", "false", "and", "or", "not",
        "in", "pairs", "ipairs", "next", "type", "tostring", "tonumber", "print",
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

/// Luaの関数
pub fn get_lua_functions(prefix: &str) -> Vec<CompletionCandidate> {
    let functions = vec![
        ("print", "print(...)", "function print(...) -> nil", "Prints values to stdout"),
        ("type", "type(value)", "function type(value) -> string", "Returns the type of a value"),
        ("tostring", "tostring(value)", "function tostring(value) -> string", "Converts a value to a string"),
        ("tonumber", "tonumber(value)", "function tonumber(value) -> number | nil", "Converts a value to a number"),
        ("pairs", "pairs(table)", "function pairs(table) -> iterator", "Returns an iterator for all key-value pairs"),
        ("ipairs", "ipairs(table)", "function ipairs(table) -> iterator", "Returns an iterator for array elements"),
        ("next", "next(table, key)", "function next(table, key) -> key, value", "Returns the next key-value pair"),
        ("getmetatable", "getmetatable(object)", "function getmetatable(object) -> table | nil", "Returns the metatable of an object"),
        ("setmetatable", "setmetatable(object, metatable)", "function setmetatable(object, metatable) -> object", "Sets the metatable of an object"),
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
