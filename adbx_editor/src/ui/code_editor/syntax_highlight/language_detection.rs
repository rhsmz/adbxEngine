/// ファイル拡張子から言語を判定
pub fn detect_programming_language_from_file_path(path: &str) -> Option<&str> {
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())?
        .to_lowercase();

    match extension.as_str() {
        "rs" => Some("Rust"),
        "lua" => Some("Lua"),
        "glsl" | "vert" | "frag" => Some("GLSL"),
        "toml" => Some("TOML"),
        "json" => Some("JSON"),
        "yaml" | "yml" => Some("YAML"),
        "md" => Some("Markdown"),
        "py" => Some("Python"),
        "js" | "jsx" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("C++"),
        "c" | "h" => Some("C"),
        "java" => Some("Java"),
        "sh" | "bash" => Some("Bash"),
        _ => None,
    }
}

/// 言語名から拡張子を取得
pub fn get_extension_from_language(lang: &str) -> &str {
    match lang {
        "Rust" => "rs",
        "Lua" => "lua",
        "Python" => "py",
        "JavaScript" => "js",
        "TypeScript" => "ts",
        "C++" => "cpp",
        "C" => "c",
        "Java" => "java",
        "Bash" => "sh",
        "GLSL" => "glsl",
        "TOML" => "toml",
        "JSON" => "json",
        "YAML" => "yaml",
        "Markdown" => "md",
        _ => lang,
    }
}
