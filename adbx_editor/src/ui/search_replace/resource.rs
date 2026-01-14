use bevy::prelude::*;

/// 検索・置換のリソース
#[derive(Resource, Default)]
pub struct SearchReplace {
    pub is_search_visible: bool,
    pub is_replace_visible: bool,
    pub search_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub use_regex: bool,
    #[allow(dead_code)]
    pub match_whole_word: bool,
    pub search_results: Vec<SearchResult>,
    pub current_result_index: usize,
    pub dialog_entity: Option<Entity>,
}

/// 検索結果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    #[allow(dead_code)]
    pub match_text: String,
}

/// 検索ダイアログを表示
pub fn show_search_dialog(search_replace: &mut ResMut<SearchReplace>) {
    search_replace.is_search_visible = true;
    search_replace.is_replace_visible = false;
}

/// 置換ダイアログを表示
pub fn show_replace_dialog(search_replace: &mut ResMut<SearchReplace>) {
    search_replace.is_search_visible = true;
    search_replace.is_replace_visible = true;
}
