use crate::ui::context_menu::{handle_search_replace_dialog, SearchReplaceDialogType};
use crate::ui::search_replace::SearchReplace;
use bevy::prelude::*;

/// 検索・置換操作のメニュー項目を処理
pub fn handle_search_replace_operation_menu_item(
    search_replace: &mut ResMut<SearchReplace>,
    operation: &str,
) {
    match operation {
        "ContextMenuFind" => {
            handle_search_replace_dialog(search_replace, SearchReplaceDialogType::Search);
        }
        "ContextMenuReplace" => {
            handle_search_replace_dialog(search_replace, SearchReplaceDialogType::Replace);
        }
        _ => {}
    }
}
