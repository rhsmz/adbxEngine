#[cfg(test)]
mod tests {
    use adbx_editor::ui::code_editor::syntax_highlight::*;

    #[test]
    fn test_detect_programming_language_from_file_path() {
        assert_eq!(
            detect_programming_language_from_file_path("test.rs"),
            Some("Rust")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.lua"),
            Some("Lua")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.py"),
            Some("Python")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.js"),
            Some("JavaScript")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.ts"),
            Some("TypeScript")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.cpp"),
            Some("C++")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.c"),
            Some("C")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.java"),
            Some("Java")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.sh"),
            Some("Bash")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.glsl"),
            Some("GLSL")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.toml"),
            Some("TOML")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.json"),
            Some("JSON")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.yaml"),
            Some("YAML")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.md"),
            Some("Markdown")
        );
        assert_eq!(
            detect_programming_language_from_file_path("test.unknown"),
            None
        );
    }

    #[test]
    fn test_get_extension_from_language() {
        assert_eq!(get_extension_from_language("Rust"), "rs");
        assert_eq!(get_extension_from_language("Lua"), "lua");
        assert_eq!(get_extension_from_language("Python"), "py");
        assert_eq!(get_extension_from_language("JavaScript"), "js");
        assert_eq!(get_extension_from_language("TypeScript"), "ts");
        assert_eq!(get_extension_from_language("C++"), "cpp");
        assert_eq!(get_extension_from_language("C"), "c");
        assert_eq!(get_extension_from_language("Java"), "java");
        assert_eq!(get_extension_from_language("Bash"), "sh");
        assert_eq!(get_extension_from_language("GLSL"), "glsl");
        assert_eq!(get_extension_from_language("TOML"), "toml");
        assert_eq!(get_extension_from_language("JSON"), "json");
        assert_eq!(get_extension_from_language("YAML"), "yaml");
        assert_eq!(get_extension_from_language("Markdown"), "md");
    }

    #[test]
    fn test_get_default_text_color() {
        let color = get_default_text_color();
        assert_eq!(color.r(), 0.9);
        assert_eq!(color.g(), 0.9);
        assert_eq!(color.b(), 0.9);
    }
}
