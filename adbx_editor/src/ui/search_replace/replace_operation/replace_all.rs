use super::super::resource::SearchReplace;
use bevy::prelude::*;

/// すべて置換を実行
pub fn perform_replace_all(
    search_replace: &mut ResMut<SearchReplace>,
    code_editor: &mut ResMut<crate::ui::code_editor::CodeEditor>,
) {
    if search_replace.search_text.is_empty() {
        return;
    }

    // active_tabの値を先に取得
    let active_tab = code_editor.active_tab;

    // 現在のファイルの内容を取得
    let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        &mut active_file.content
    } else {
        &mut code_editor.content
    };

    // すべて置換
    if search_replace.use_regex {
        // 正規表現置換
        if let Ok(re) = regex::Regex::new(&search_replace.search_text) {
            *current_content = re
                .replace_all(current_content, &search_replace.replace_text)
                .to_string();
        }
    } else {
        // 通常の文字列置換
        *current_content =
            current_content.replace(&search_replace.search_text, &search_replace.replace_text);
    }

    // 変更をマーク
    if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
        active_file.modified = true;
    }

    // UI更新を促す
    code_editor.content_entity = None;
}
