use bevy::prelude::*;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use std::sync::LazyLock;

/// シンタックスハイライト用のSyntaxSet（グローバルに一度だけ読み込む）
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(|| {
    SyntaxSet::load_defaults_newlines()
});

/// シンタックスハイライト用のTheme（グローバルに一度だけ読み込む）
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(|| {
    ThemeSet::load_defaults()
});

/// syntectのStyleをBevyのColorに変換
fn style_to_color(style: &Style) -> Color {
    let r = style.foreground.r as f32 / 255.0;
    let g = style.foreground.g as f32 / 255.0;
    let b = style.foreground.b as f32 / 255.0;
    Color::srgb(r, g, b)
}

/// Luaテキストを行ごとにシンタックスハイライトして返す
pub fn highlight_lua_text(text: &str) -> Vec<Vec<(String, Color)>> {
    let syntax_set = &*SYNTAX_SET;
    
    // Luaシンタックスを取得
    let syntax = syntax_set.find_syntax_by_name("Lua")
        .or_else(|| syntax_set.find_syntax_by_extension("lua"))
        .or_else(|| syntax_set.find_syntax_by_extension("Lua"));
    
    // シンタックスが見つからない場合は通常のテキストとして返す
    let syntax = match syntax {
        Some(s) => s,
        None => {
            return text.lines()
                .map(|line| vec![(line.to_string(), Color::srgb(0.9, 0.9, 0.9))])
                .collect();
        }
    };
    
    // ハイライト処理
    let theme_set = &*THEME_SET;
    let theme = theme_set.themes.get("base16-ocean.dark")
        .or_else(|| theme_set.themes.values().next())
        .expect("No theme available");
    
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = Vec::new();
    
    for line in text.lines() {
        let mut line_tokens = Vec::new();
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, syntax_set).unwrap_or_default();
        
        for (style, text) in ranges {
            line_tokens.push((text.to_string(), style_to_color(&style)));
        }
        
        // 空行の場合は空のトークンリストを追加
        if line_tokens.is_empty() {
            line_tokens.push((String::new(), Color::srgb(0.9, 0.9, 0.9)));
        }
        
        result.push(line_tokens);
    }
    
    result
}
