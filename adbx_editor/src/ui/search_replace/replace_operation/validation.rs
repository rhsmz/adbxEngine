use super::super::resource::SearchReplace;

/// 置換の検証
#[allow(dead_code)]
pub fn validate_replace(search_replace: &SearchReplace) -> Result<(), String> {
    if search_replace.search_text.is_empty() {
        return Err("検索テキストが空です".to_string());
    }

    if search_replace.use_regex {
        if regex::Regex::new(&search_replace.search_text).is_err() {
            return Err("無効な正規表現です".to_string());
        }
    }

    Ok(())
}
