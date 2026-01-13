use bevy::prelude::*;
use std::collections::HashMap;

/// Entity IDのマッピングを作成
pub fn create_entity_id_mapping(
    entities: &Query<(Entity, &Name), With<Transform>>,
) -> HashMap<Entity, u32> {
    let mut entity_map: HashMap<Entity, u32> = HashMap::new();
    let mut next_id = 0u32;
    
    for (entity, _) in entities.iter() {
        entity_map.insert(entity, next_id);
        next_id += 1;
    }
    
    entity_map
}

/// 親子関係の逆引きマップを作成（子から親を取得）
pub fn create_child_to_parent_map(
    entities: &Query<(Entity, &Name), With<Transform>>,
    children_query: &Query<&Children>,
) -> HashMap<Entity, Entity> {
    let mut child_to_parent: HashMap<Entity, Entity> = HashMap::new();
    
    for (entity, _) in entities.iter() {
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                child_to_parent.insert(child, entity);
            }
        }
    }
    
    child_to_parent
}
