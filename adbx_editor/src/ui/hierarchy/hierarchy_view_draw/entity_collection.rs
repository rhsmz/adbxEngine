use super::super::hierarchy_view_resource::HierarchyView;
use bevy::prelude::*;

/// 表示可能なエンティティのリストを収集（展開された状態で）
pub fn collect_expanded_hierarchy_entities(
    root_entities: &[Entity],
    hierarchy_view: &HierarchyView,
    children_query: &Query<&Children>,
    name_query: &Query<&Name>,
) -> Vec<Entity> {
    let mut visible = Vec::new();

    fn collect_recursive(
        entity: Entity,
        hierarchy_view: &HierarchyView,
        children_query: &Query<&Children>,
        name_query: &Query<&Name>,
        visible: &mut Vec<Entity>,
    ) {
        // エンティティ名がある場合のみ追加
        if name_query.get(entity).is_ok() {
            visible.push(entity);
        }

        // 展開されている場合、子エンティティも追加
        if hierarchy_view.expanded_entities.contains(&entity) {
            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    collect_recursive(child, hierarchy_view, children_query, name_query, visible);
                }
            }
        }
    }

    for root_entity in root_entities {
        collect_recursive(
            *root_entity,
            hierarchy_view,
            children_query,
            name_query,
            &mut visible,
        );
    }

    visible
}

/// インデントレベルを計算
pub fn calculate_hierarchy_item_indent_level(
    entity: Entity,
    root_entities: &[Entity],
    children_query: &Query<&Children>,
    hierarchy_view: &HierarchyView,
) -> u32 {
    // ルートエンティティかどうかをチェック
    if root_entities.contains(&entity) {
        return 0;
    }

    // 親エンティティを探す
    for root in root_entities {
        if let Some(level) = find_entity_level(*root, entity, children_query, hierarchy_view, 0) {
            return level;
        }
    }

    0
}

/// エンティティの階層レベルを再帰的に検索
fn find_entity_level(
    current: Entity,
    target: Entity,
    children_query: &Query<&Children>,
    hierarchy_view: &HierarchyView,
    current_level: u32,
) -> Option<u32> {
    if current == target {
        return Some(current_level);
    }

    if hierarchy_view.expanded_entities.contains(&current) {
        if let Ok(children) = children_query.get(current) {
            for child in children.iter() {
                if let Some(level) = find_entity_level(
                    child,
                    target,
                    children_query,
                    hierarchy_view,
                    current_level + 1,
                ) {
                    return Some(level);
                }
            }
        }
    }

    None
}
