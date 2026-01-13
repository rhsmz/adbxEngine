use bevy::prelude::*;
use super::resource::{SearchReplace, SearchResult};
use super::replace_operation::{perform_replace, perform_replace_all};

/// 検索を実行
pub fn perform_search(
    search_replace: &mut ResMut<SearchReplace>,
    code_editor: &ResMut<crate::ui::code_editor::CodeEditor>,
) {
    if search_replace.search_text.is_empty() {
        return;
    }
    
    // active_tabの値を先に取得
    let active_tab = code_editor.active_tab;
    
    // 現在のファイルの内容を取得
    let content = if let Some(active_file) = code_editor.open_files.get(active_tab) {
        &active_file.content
    } else {
        &code_editor.content
    };
    
    search_replace.search_results.clear();
    
    // 検索を実行
    if search_replace.use_regex {
        // 正規表現検索（簡易実装）
        if let Ok(re) = regex::Regex::new(&search_replace.search_text) {
            for (line_num, line) in content.lines().enumerate() {
                for mat in re.find_iter(line) {
                    search_replace.search_results.push(SearchResult {
                        line: line_num,
                        column: mat.start(),
                        length: mat.end() - mat.start(),
                        match_text: mat.as_str().to_string(),
                    });
                }
            }
        }
    } else {
        // 通常の文字列検索
        let search_pattern = if search_replace.case_sensitive {
            search_replace.search_text.clone()
        } else {
            search_replace.search_text.to_lowercase()
        };
        
        for (line_num, line) in content.lines().enumerate() {
            let search_line = if search_replace.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };
            
            let mut start = 0;
            while let Some(pos) = search_line[start..].find(&search_pattern) {
                let actual_pos = start + pos;
                search_replace.search_results.push(SearchResult {
                    line: line_num,
                    column: actual_pos,
                    length: search_pattern.len(),
                    match_text: line[actual_pos..actual_pos + search_pattern.len()].to_string(),
                });
                start = actual_pos + search_pattern.len();
            }
        }
    }
    
    if !search_replace.search_results.is_empty() {
        search_replace.current_result_index = 0;
        bevy::log::info!("Found {} matches", search_replace.search_results.len());
    } else {
        bevy::log::info!("No matches found");
    }
}

/// 検索・置換ダイアログの入力処理
pub fn handle_search_replace_input(
    mut search_replace: ResMut<SearchReplace>,
    mut code_editor: ResMut<crate::ui::code_editor::CodeEditor>,
    interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Escapeキーでダイアログを閉じる
    if keyboard_input.just_pressed(KeyCode::Escape) {
        search_replace.is_search_visible = false;
        search_replace.is_replace_visible = false;
        return;
    }
    
    // Ctrl+F: 検索ダイアログを開く
    if (keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight))
        && keyboard_input.just_pressed(KeyCode::KeyF) {
        super::resource::show_search_dialog(&mut search_replace);
        return;
    }
    
    // Ctrl+H: 置換ダイアログを開く
    if (keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight))
        && keyboard_input.just_pressed(KeyCode::KeyH) {
        super::resource::show_replace_dialog(&mut search_replace);
        return;
    }
    
    // ボタンのクリック処理
    for (interaction, name) in interaction_query.iter() {
        if *interaction == bevy::ui::Interaction::Pressed {
            let name_str = name.as_str();
            
            match name_str {
                "SearchButton" => {
                    perform_search(&mut search_replace, &code_editor);
                }
                "ReplaceButton" => {
                    perform_replace(&mut search_replace, &mut code_editor);
                }
                "ReplaceAllButton" => {
                    perform_replace_all(&mut search_replace, &mut code_editor);
                }
                "CloseButton" => {
                    search_replace.is_search_visible = false;
                    search_replace.is_replace_visible = false;
                }
                "CaseSensitiveCheckbox" => {
                    search_replace.case_sensitive = !search_replace.case_sensitive;
                }
                _ => {}
            }
        }
    }
}