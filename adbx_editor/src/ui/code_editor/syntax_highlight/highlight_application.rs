use super::language_detection::get_extension_from_language;
use super::style_application::{convert_syntect_style_to_bevy_color, get_default_text_color};
use bevy::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

/// シンタックスハイライト用のSyntaxSet（グローバルに一度だけ読み込む）
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(|| SyntaxSet::load_defaults_newlines());

/// シンタックスハイライト用のTheme（グローバルに一度だけ読み込む）
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(|| ThemeSet::load_defaults());

/// テキストを行ごとにシンタックスハイライトして返す（改善版：キャッシュ対応）
pub fn highlight_code_text_by_line(
    text: &str,
    language: Option<&str>,
    cache: &mut std::collections::HashMap<usize, Vec<(String, Color)>>,
    cache_hash: &mut u64,
) -> Vec<Vec<(String, Color)>> {
    // コンテンツのハッシュを計算
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let current_hash = hasher.finish();

    // キャッシュが有効な場合は再利用
    if *cache_hash == current_hash && !cache.is_empty() {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = Vec::new();
        for i in 0..lines.len() {
            if let Some(tokens) = cache.get(&i) {
                result.push(tokens.clone());
            } else {
                // キャッシュにない行は通常のテキストとして返す
                result.push(vec![(lines[i].to_string(), get_default_text_color())]);
            }
        }
        return result;
    }

    // キャッシュをクリア
    cache.clear();
    *cache_hash = current_hash;

    let syntax_set = &*SYNTAX_SET;

    // 言語に応じたシンタックスを取得
    let syntax = if let Some(lang) = language {
        // 言語名から直接検索
        syntax_set
            .find_syntax_by_name(lang)
            // 拡張子から検索（言語名を拡張子として扱う）
            .or_else(|| {
                let ext = get_extension_from_language(lang);
                syntax_set.find_syntax_by_extension(ext)
            })
    } else {
        None
    };

    // シンタックスが見つからない場合は通常のテキストとして返す
    let syntax = match syntax {
        Some(s) => s,
        None => {
            return text
                .lines()
                .map(|line| vec![(line.to_string(), get_default_text_color())])
                .collect();
        }
    };

    // ハイライト処理
    let theme_set = &*THEME_SET;
    let theme = theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.values().next())
        .expect("No theme available");

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let mut line_tokens = Vec::new();
        let ranges: Vec<(Style, &str)> = highlighter
            .highlight_line(line, syntax_set)
            .unwrap_or_default();

        for (style, text) in ranges {
            line_tokens.push((
                text.to_string(),
                convert_syntect_style_to_bevy_color(&style),
            ));
        }

        // 空行の場合は空のトークンリストを追加
        if line_tokens.is_empty() {
            line_tokens.push((String::new(), get_default_text_color()));
        }

        // キャッシュに保存
        cache.insert(line_idx, line_tokens.clone());
        result.push(line_tokens);
    }

    result
}

/// シンタックスハイライト処理（`with_children`の外で実行）
pub fn apply_syntax_highlighting_to_code(
    content_text: &str,
    language: Option<&str>,
    code_editor: &mut crate::ui::code_editor::resource::CodeEditor,
) -> Vec<Vec<(String, Color)>> {
    let cache = &mut code_editor.syntax_highlight_cache;
    let cache_hash = &mut code_editor.syntax_highlight_cache_hash;
    highlight_code_text_by_line(content_text, language, cache, cache_hash)
}
