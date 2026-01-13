use bevy::prelude::*;

/// Luaのトークンタイプ
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LuaTokenType {
    Keyword,
    String,
    Comment,
    Number,
    Identifier,
    Operator,
    Normal,
}

/// Luaのトークン
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct LuaToken {
    text: String,
    token_type: LuaTokenType,
}

/// Luaのキーワードリスト
#[allow(dead_code)]
const LUA_KEYWORDS: &[&str] = &[
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
    "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

/// 文字列をLuaトークンに分割
#[allow(dead_code)]
fn tokenize_lua_line(line: &str) -> Vec<LuaToken> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut string_char = '\0';
    let mut in_comment = false;
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();

    while i < chars.len() {
        let ch = chars[i];

        if in_comment {
            current.push(ch);
            i += 1;
            continue;
        }

        if in_string {
            current.push(ch);
            if ch == string_char && (i == 0 || chars[i - 1] != '\\') {
                tokens.push(LuaToken {
                    text: current.clone(),
                    token_type: LuaTokenType::String,
                });
                current.clear();
                in_string = false;
            }
            i += 1;
            continue;
        }

        match ch {
            '"' | '\'' => {
                if !current.is_empty() {
                    tokens.push(parse_token(&current));
                    current.clear();
                }
                in_string = true;
                string_char = ch;
                current.push(ch);
            }
            '-' if i + 1 < chars.len() && chars[i + 1] == '-' => {
                if !current.is_empty() {
                    tokens.push(parse_token(&current));
                    current.clear();
                }
                in_comment = true;
                current.push(ch);
            }
            c if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(parse_token(&current));
                    current.clear();
                }
            }
            c if is_operator(c) => {
                if !current.is_empty() {
                    tokens.push(parse_token(&current));
                    current.clear();
                }
                tokens.push(LuaToken {
                    text: c.to_string(),
                    token_type: LuaTokenType::Operator,
                });
            }
            _ => {
                current.push(ch);
            }
        }
        i += 1;
    }

    if !current.is_empty() {
        if in_comment {
            tokens.push(LuaToken {
                text: current,
                token_type: LuaTokenType::Comment,
            });
        } else if in_string {
            tokens.push(LuaToken {
                text: current,
                token_type: LuaTokenType::String,
            });
        } else {
            tokens.push(parse_token(&current));
        }
    }

    tokens
}

/// トークンを解析してタイプを決定
#[allow(dead_code)]
fn parse_token(text: &str) -> LuaToken {
    if LUA_KEYWORDS.contains(&text) {
        LuaToken {
            text: text.to_string(),
            token_type: LuaTokenType::Keyword,
        }
    } else if text.parse::<f64>().is_ok() || text.parse::<i64>().is_ok() {
        LuaToken {
            text: text.to_string(),
            token_type: LuaTokenType::Number,
        }
    } else if text.chars().all(|c| c.is_alphanumeric() || c == '_') {
        LuaToken {
            text: text.to_string(),
            token_type: LuaTokenType::Identifier,
        }
    } else {
        LuaToken {
            text: text.to_string(),
            token_type: LuaTokenType::Normal,
        }
    }
}

/// 文字が演算子かどうか
#[allow(dead_code)]
fn is_operator(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '^'
            | '#'
            | '='
            | '<'
            | '>'
            | '('
            | ')'
            | '{'
            | '}'
            | '['
            | ']'
            | ';'
            | ':'
            | ','
            | '.'
    )
}

/// トークンタイプに対応する色を取得
#[allow(dead_code)]
fn get_token_color(token_type: LuaTokenType) -> Color {
    match token_type {
        LuaTokenType::Keyword => Color::srgb(0.8, 0.4, 0.8), // 紫
        LuaTokenType::String => Color::srgb(0.6, 0.8, 0.6),  // 緑
        LuaTokenType::Comment => Color::srgb(0.5, 0.5, 0.5), // グレー
        LuaTokenType::Number => Color::srgb(0.8, 0.6, 0.4),  // オレンジ
        LuaTokenType::Identifier => Color::srgb(0.9, 0.9, 0.9), // 白
        LuaTokenType::Operator => Color::srgb(0.7, 0.7, 0.7), // ライトグレー
        LuaTokenType::Normal => Color::srgb(0.9, 0.9, 0.9),  // 白
    }
}
