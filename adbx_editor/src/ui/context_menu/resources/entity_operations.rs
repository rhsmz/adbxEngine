use super::types::EntityOperation;
use bevy::prelude::*;

/// エンティティ操作（作成、削除、複製）
pub fn handle_entity_operations(
    commands: &mut Commands,
    selection: &mut ResMut<crate::systems::selection::Selection>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
    children_query: &Query<&Children>,
    operation: EntityOperation,
) {
    match operation {
        EntityOperation::Create => {
            create_entity(commands, selection);
        }
        EntityOperation::Delete => {
            delete_entities(commands, selection);
        }
        EntityOperation::Duplicate => {
            duplicate_entities(
                commands,
                selection,
                transform_query,
                name_query,
                children_query,
            );
        }
    }
}

/// エンティティを作成
fn create_entity(
    commands: &mut Commands,
    selection: &mut ResMut<crate::systems::selection::Selection>,
) {
    let entity_name = format!(
        "NewEntity_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let new_entity = commands
        .spawn((Name::new(entity_name.clone()), Transform::default()))
        .id();

    selection.selected_entities.clear();
    selection.selected_entities.push(new_entity);
    bevy::log::info!("Created entity: {}", entity_name);
}

/// エンティティを削除
fn delete_entities(
    commands: &mut Commands,
    selection: &mut ResMut<crate::systems::selection::Selection>,
) {
    let entities_to_delete: Vec<Entity> = selection.selected_entities.iter().copied().collect();
    for entity in entities_to_delete {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
            bevy::log::info!("Deleted entity: {:?}", entity);
        }
    }
    selection.selected_entities.clear();
}

/// エンティティを複製
fn duplicate_entities(
    commands: &mut Commands,
    selection: &mut ResMut<crate::systems::selection::Selection>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
    children_query: &Query<&Children>,
) {
    let entities_to_duplicate: Vec<Entity> = selection.selected_entities.iter().copied().collect();
    let mut new_entities = Vec::new();

    for entity in entities_to_duplicate {
        if let Ok(name) = name_query.get(entity) {
            let new_name = format!("{}_Copy", name.as_str());
            let transform = transform_query.get(entity).copied().unwrap_or_default();

            let new_entity = commands
                .spawn((Name::new(new_name.clone()), transform))
                .id();

            // 子エンティティも複製（簡易実装）
            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    if let Ok(child_name) = name_query.get(child) {
                        let child_transform =
                            transform_query.get(child).copied().unwrap_or_default();
                        let new_child = commands
                            .spawn((
                                Name::new(format!("{}_Copy", child_name.as_str())),
                                child_transform,
                            ))
                            .id();
                        commands.entity(new_entity).add_child(new_child);
                    }
                }
            }

            new_entities.push(new_entity);
            bevy::log::info!("Duplicated entity: {} -> {}", name.as_str(), new_name);
        }
    }

    if !new_entities.is_empty() {
        selection.selected_entities = new_entities;
    }
}
