use crate::systems::selection::Selection;
use bevy::prelude::*;

/// シーン操作メニュー項目を処理
pub fn handle_scene_operation_menu_item(
    selection: &Selection,
    transform_query: &Query<&Transform>,
    operation: &str,
) {
    match operation {
        "ContextMenuFocusSelection" => {
            // TODO: 選択エンティティにフォーカスする処理を実装
            if let Some(entity) = selection.selected_entities.first() {
                if transform_query.get(*entity).is_ok() {
                    bevy::log::info!("Focus selection requested for entity {:?}", entity);
                }
            }
        }
        _ => {}
    }
}
