use crate::ui::code_editor::{CompletionCandidate, CompletionKind};

/// Pythonのキーワード
pub fn get_python_keywords(prefix: &str) -> Vec<CompletionCandidate> {
    let keywords = vec![
        "def", "class", "if", "elif", "else", "for", "while", "break", "continue", "return",
        "yield", "import", "from", "as", "try", "except", "finally", "raise", "assert", "with",
        "pass", "lambda", "None", "True", "False", "and", "or", "not", "in", "is", "del", "global",
        "nonlocal",
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

/// Pythonの関数
pub fn get_python_functions(prefix: &str) -> Vec<CompletionCandidate> {
    let functions = vec![
        (
            "print",
            "print(*args, **kwargs)",
            "def print(*args, sep=' ', end='\\n', file=sys.stdout) -> None",
            "Prints values to stdout",
        ),
        (
            "len",
            "len(iterable)",
            "def len(obj) -> int",
            "Returns the length of an object",
        ),
        (
            "range",
            "range(start, stop, step)",
            "def range(start, stop=None, step=1) -> range",
            "Returns a sequence of numbers",
        ),
        (
            "str",
            "str(object)",
            "def str(object='') -> str",
            "Converts an object to a string",
        ),
        (
            "int",
            "int(x)",
            "def int(x=0, base=10) -> int",
            "Converts a value to an integer",
        ),
        (
            "float",
            "float(x)",
            "def float(x=0.0) -> float",
            "Converts a value to a float",
        ),
        (
            "list",
            "list(iterable)",
            "def list(iterable=()) -> list",
            "Creates a list from an iterable",
        ),
        (
            "dict",
            "dict(**kwargs)",
            "def dict(**kwargs) -> dict",
            "Creates a dictionary",
        ),
        (
            "tuple",
            "tuple(iterable)",
            "def tuple(iterable=()) -> tuple",
            "Creates a tuple from an iterable",
        ),
        (
            "set",
            "set(iterable)",
            "def set(iterable=()) -> set",
            "Creates a set from an iterable",
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
