use super::super::resource::{SearchReplace, SearchResult};
use bevy::prelude::*;

/// 置換を実行
pub fn perform_replace(
    search_replace: &mut ResMut<SearchReplace>,
    code_editor: &mut ResMut<crate::ui::code_editor::CodeEditor>,
) {
    if search_replace.search_text.is_empty() || search_replace.search_results.is_empty() {
        return;
    }

    // active_tabの値を先に取得
    let active_tab = code_editor.active_tab;

    // 現在の結果を置換
    if let Some(result) = search_replace
        .search_results
        .get(search_replace.current_result_index)
    {
        // 現在のファイルの内容を取得
        let current_content = if let Some(active_file) = code_editor.open_files.get_mut(active_tab)
        {
            &mut active_file.content
        } else {
            &mut code_editor.content
        };

        // 行を取得して置換
        let lines: Vec<&str> = current_content.lines().collect();
        if let Some(line) = lines.get(result.line) {
            let new_line = format!(
                "{}{}{}",
                &line[..result.column],
                &search_replace.replace_text,
                &line[result.column + result.length..]
            );

            // 行を置き換え
            let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
            new_lines[result.line] = new_line;
            *current_content = new_lines.join("\n");

            // 変更をマーク
            if let Some(active_file) = code_editor.open_files.get_mut(active_tab) {
                active_file.modified = true;
            }

            // UI更新を促す
            code_editor.content_entity = None;

            // 次の結果に移動
            search_replace.current_result_index =
                (search_replace.current_result_index + 1) % search_replace.search_results.len();
        }
    }
}
