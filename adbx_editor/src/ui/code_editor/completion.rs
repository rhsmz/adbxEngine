use crate::ui::code_editor::resource::CodeEditor;
use crate::ui::code_editor::state::OpenFile;

/// 補完を承認（選択された候補を挿入）
pub fn accept_selected_completion_candidate(code_editor: &mut CodeEditor) {
    if !code_editor.completion_state.is_visible {
        return;
    }
    
    if let Some(candidate) = code_editor.completion_state.candidates.get(code_editor.completion_state.selected_index) {
        // 現在のコンテンツを取得
        let current_content = if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
            &mut active_file.content
        } else {
            &mut code_editor.content
        };
        
        // プレフィックスを削除
        let prefix_start = code_editor.completion_state.trigger_position.saturating_sub(code_editor.completion_state.prefix.len());
        let prefix_end = code_editor.completion_state.trigger_position;
        
        if prefix_start < prefix_end && prefix_end <= current_content.len() {
            current_content.replace_range(prefix_start..prefix_end, &candidate.insert_text);
            code_editor.cursor_position = prefix_start + candidate.insert_text.len();
            
            // 変更をマーク
            if let Some(active_file) = code_editor.open_files.get_mut(code_editor.active_tab) {
                active_file.modified = true;
            }
        }
        
        // 補完を非表示にする
        code_editor.completion_state.is_visible = false;
        code_editor.content_entity = None;
    }
}

/// コード補完をトリガー
pub fn trigger_code_completion_for_current_position(code_editor: &mut CodeEditor) {
    // 現在のファイルの内容を取得
    let (content, cursor_pos) = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        (&active_file.content, code_editor.cursor_position)
    } else {
        (&code_editor.content, code_editor.cursor_position)
    };
    
    // カーソル位置から前の文字列を取得（補完のプレフィックス）
    let prefix_start = content[..cursor_pos.min(content.len())]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    
    let prefix = content[prefix_start..cursor_pos.min(content.len())].to_string();
    
    // プレフィックスが空または短すぎる場合は補完を表示しない
    if prefix.is_empty() || prefix.len() < 1 {
        code_editor.completion_state.is_visible = false;
        return;
    }
    
    // 言語を判定
    let language = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        crate::ui::code_editor::syntax_highlight::detect_programming_language_from_file_path(&active_file.path)
    } else if let Some(ref current_file) = code_editor.current_file {
        crate::ui::code_editor::syntax_highlight::detect_programming_language_from_file_path(current_file)
    } else {
        None
    };
    
    // 補完候補を取得
    let candidates = crate::ui::code_completion::get_completion_candidates(
        &prefix,
        language,
        content,
        cursor_pos,
    );
    
    if !candidates.is_empty() {
        code_editor.completion_state.is_visible = true;
        code_editor.completion_state.candidates = candidates;
        code_editor.completion_state.selected_index = 0;
        code_editor.completion_state.trigger_position = cursor_pos;
        code_editor.completion_state.prefix = prefix;
    } else {
        code_editor.completion_state.is_visible = false;
    }
}
