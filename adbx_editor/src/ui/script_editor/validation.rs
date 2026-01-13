use super::{ScriptEditor, ScriptError};
use bevy::prelude::*;
use mlua::Lua;

/// スクリプトのバリデーション
pub fn validate_script(script_editor: &mut ScriptEditor) {
    script_editor.errors.clear();

    if script_editor.content.is_empty() {
        return;
    }

    // mluaを使用してスクリプトをコンパイルしてエラーを検出
    let lua = Lua::new();
    match lua.load(&script_editor.content).exec() {
        Ok(_) => {
            // エラーなし
        }
        Err(e) => {
            // エラー情報を解析
            let error_msg = e.to_string();

            // エラーメッセージから行番号を抽出（簡易実装）
            // 実際のmluaエラーは "attempt to call a nil value (line X)" のような形式
            let line = extract_line_number(&error_msg).unwrap_or(1);

            script_editor.errors.push(ScriptError {
                line,
                message: error_msg,
                column: None,
            });
        }
    }
}

/// スクリプトのエラーを検出するシステム関数
pub fn validate_script_system(mut script_editor: ResMut<ScriptEditor>) {
    validate_script(script_editor.as_mut());
}

/// エラーメッセージから行番号を抽出
fn extract_line_number(error_msg: &str) -> Option<usize> {
    // "line X" や "(line X)" のようなパターンを検索
    if let Some(pos) = error_msg.rfind("line ") {
        let rest = &error_msg[pos + 5..];
        if let Some(end) = rest.find(|c: char| !c.is_ascii_digit()) {
            rest[..end].parse().ok()
        } else {
            rest.parse().ok()
        }
    } else {
        None
    }
}
