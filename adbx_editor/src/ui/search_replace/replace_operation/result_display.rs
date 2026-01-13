use super::super::resource::SearchReplace;

/// 置換結果の表示
pub fn display_replace_result(
    _search_replace: &SearchReplace,
    replaced_count: usize,
) {
    if replaced_count > 0 {
        bevy::log::info!("{}件の置換を実行しました", replaced_count);
    } else {
        bevy::log::info!("置換する項目が見つかりませんでした");
    }
}
