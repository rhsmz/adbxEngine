use super::super::{handle_entity_operations, EntityOperation};
use crate::systems::selection::Selection;
use bevy::prelude::*;

/// エンティティ操作のメニュー項目を処理
pub fn handle_entity_operation_menu_item(
    commands: &mut Commands,
    selection: &mut ResMut<Selection>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
    children_query: &Query<&Children>,
    operation: EntityOperation,
) {
    handle_entity_operations(
        commands,
        selection,
        transform_query,
        name_query,
        children_query,
        operation,
    );
}
